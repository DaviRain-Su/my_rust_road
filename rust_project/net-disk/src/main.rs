
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, Ordering};


// 线程池的实现
pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

pub type Task = Box<dyn FnBox + Send >;

// pub struct ThreadPool{
//     tx: Option<Sender<Task>>,
//     handles: Option<Vec<thread::JoinHandle<()>>>,
// }

// impl ThreadPool {
//     pub fn new(number: usize) -> Self {
//         let (tx, rx) =  channel::<Task>();
//         let mut handles = vec![];

//         let arx = Arc::new(Mutex::new(rx));

//         for _ in 0..number {
//             let arx = arx.clone();
//             let handle = thread::spawn(move || {
//                 while let Ok(task) = arx.lock().unwrap().recv() {
//                     task.call_box();
//                 }
//             });
//             handles.push(handle);
//         }

//         Self {
//             tx: Some(tx),
//             handles: Some(handles),
//         }
//     }
// }

fn main() {
    let handles = ThreadPool::new(12);
    let sender = handles.tx.unwrap().clone();
    for _ in 0..40 {
        sender.send(Box::new(|| {
            println!("Hello world");
        })).unwrap();
    }

    for id in handles.handles.unwrap() {
        id.join().unwrap();
    }
    println!("Hello, world!");
}
