use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// 线程池的实现
pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

pub type Task<'a> = Box<dyn FnBox + Send + 'a>;

struct ThreadPoolSharedData {
    name: Option<String>,                         // 标记线程名称，可有可无
    job_receiver: Mutex<Receiver<Task<'static>>>, // 存储从Channel中接手任务的接收端rx
    empty_trigger: Mutex<()>,                     // 空锁
    empty_condvar: Condvar,                       // 空的条件变量
    queued_count: AtomicUsize,                    // 线程池中的总队列数
    active_count: AtomicUsize,                    //正在执行任务的工作线程数
    max_thread_count: AtomicUsize,                // 线程池允许的最大工作线程数
    panic_count: AtomicUsize,                     // 记录线程池中发生恐慌的工作线程
    stack_size: Option<usize>,                    // 用于设置工作线程栈的大小，可有可无
}

impl ThreadPoolSharedData {
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0 || self.active_count.load(Ordering::SeqCst) > 0
    }

    fn no_work_notify_all(&self) {
        if !self.has_work() {
            *self
                .empty_trigger
                .lock()
                .expect("Unable to notify all joining threads");
            self.empty_condvar.notify_all();
        }
    }
}

pub struct ThreadPool {
    jobs: Sender<Task<'static>>,
    shared_data: Arc<ThreadPoolSharedData>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        Builder::new().num_threads(num_threads).build()
    }
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.shared_data.queued_count.fetch_add(1, Ordering::SeqCst);
        self.jobs
            .send(Box::new(job))
            .expect("unable to send job into queue.");
    }

    pub fn join(&self) {
        if self.shared_data.has_work() == false {
            return ();
        }
        let mut lock = self.shared_data.empty_trigger.lock().unwrap();
        while self.shared_data.has_work() {
            lock = self.shared_data.empty_condvar.wait(lock).unwrap();
        }
    }
}

#[derive(Clone, Default)]
pub struct Builder {
    num_threads: Option<usize>,
    thread_name: Option<String>,
    thread_stack_size: Option<usize>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            num_threads: None,
            thread_name: None,
            thread_stack_size: None,
        }
    }

    pub fn num_threads(mut self, num_threads: usize) -> Self {
        assert!(num_threads > 0);
        self.num_threads = Some(num_threads);
        self
    }

    pub fn build(self) -> ThreadPool {
        let (tx, rx) = channel::<Task<'static>>();
        let num_threads = self.num_threads.unwrap_or_else(num_cpus::get);

        let shared_data = Arc::new(ThreadPoolSharedData {
            name: self.thread_name,
            job_receiver: Mutex::new(rx),
            empty_condvar: Condvar::new(),
            empty_trigger: Mutex::new(()),
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            max_thread_count: AtomicUsize::new(num_threads),
            panic_count: AtomicUsize::new(0),
            stack_size: self.thread_stack_size,
        });

        for _ in 0..num_threads {
            spawn_in_pool(shared_data.clone());
        }

        ThreadPool {
            jobs: tx,
            shared_data: shared_data,
        }
    }
}

fn spawn_in_pool(shared_data: Arc<ThreadPoolSharedData>) {
    let mut builder = thread::Builder::new();

    if let Some(ref name) = shared_data.name {
        builder = builder.name(name.clone());
    }
    if let Some(ref stack_size) = shared_data.stack_size {
        builder = builder.stack_size(stack_size.to_owned());
    }

    builder
        .spawn(move || {
            let sentinel = Sentinel::new(&shared_data);

            loop {
                let thread_counter_val = shared_data.active_count.load(Ordering::Acquire);
                let max_thread_count = shared_data.max_thread_count.load(Ordering::Relaxed);

                if thread_counter_val >= max_thread_count {
                    break;
                }

                let message = {
                    let lock = shared_data
                        .job_receiver
                        .lock()
                        .expect("unable to lock job_receiver");
                    lock.recv()
                };

                let job = match message {
                    Ok(job) => job,
                    Err(..) => break,
                };

                shared_data.queued_count.fetch_sub(1, Ordering::SeqCst);
                shared_data.active_count.fetch_add(1, Ordering::SeqCst);
                job.call_box();
                shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
                shared_data.no_work_notify_all();
            }

            sentinel.cancel();
        })
        .unwrap();
}

struct Sentinel<'a> {
    shared_data: &'a Arc<ThreadPoolSharedData>,
    active: bool,
}

impl<'a> Sentinel<'a> {
    fn new(shared_data: &'a Arc<ThreadPoolSharedData>) -> Self {
        Self {
            shared_data,
            active: true,
        }
    }

    fn cancel(mut self) {
        self.active = false;
    }
}

impl<'a> Drop for Sentinel<'a> {
    fn drop(&mut self) {
        if self.active {
            self.shared_data.active_count.fetch_sub(1, Ordering::SeqCst);
            if thread::panicking() {
                self.shared_data.panic_count.fetch_add(1, Ordering::SeqCst);
            }
            self.shared_data.no_work_notify_all();
            spawn_in_pool(self.shared_data.clone());
        }
    }
}
