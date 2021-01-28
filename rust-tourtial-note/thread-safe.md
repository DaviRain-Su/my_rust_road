# Rust 线程安全

Rust不经在没有自动垃圾回收的条件下实现了内存安全，而且实现了线程安全。Rust编译器可以在编译阶段避免所有的数据竞争问题。

## 什么是线程

线程是操作系统能够进行调度的最小单位，他是进程中的实际运作单位，每个进程至少包含一个线程。

多线程的优势：

- 容量利用多核优势
- 比单线程反应更敏捷，比多线程资源共享更容易。

线程的调度是不受我们控制的，即便线程1和线程2内部的执行流程不变，只要调度时机发生了变化，结果也会不同。

## 启动线程

Rust标准库中与线程相关的内容在std::thread模块中，Rust中的线程是对操作系统线程的直接封装。

创建线程的方法：

```rust
use std::thread;
thread::spawn(move || {
	// logic
});
```

默认情况下，新创建的子线程与原来父线程的分离的关系。也就说，子线程可以在父线程结束后继续存在。除非父线程是主线程。

这是因为，如果一个进程的主线程也退出了，这个进程就会终止。其他所有的线程也会随之结束。

如果我们需要等待子线程执行结束，那么可以使用join方法：

```rust
use std::thread;

// child type is JoinHandle<T>, 
let child  = thread::spawn( move || {
	// logic
});

// parent thread wait chile thread end 
let res = child.join();
```

如果我们需要为子线程指定更多的参数信息，那么在创建的时候可以使用Builder模式

```rust
use std::thread;

thread::Builder::new().name("child".to_string()).spawn(move || {
  println!("Hello, world!");
});
```



- Thread::sleep(dur: Duration)
  - 使得当前线程等待一段时间继续执行。在等待的时间内，线程调度器会调度其他的线程执行、
- thread::yield_now()
  - 放弃当前线程的执行，要求线程调度器执行线程切换
- Thread::current()
  - 获得当前的线程
- Thread::park() 
  - 暂停当前线程，进入等待状态，当thread::thread::unpark(&self)方法被调用的时候，这个线程可以被恢复执行。
- thread::Thread::unpark(&self)
  - 恢复一个线程的执行

```rust
use std::thread;
use std::time::Dration;

let t = thread::Builder::new()
  .name("child2".to_string())
  .spawn(move || {
    println!("enter child thread.");
    thread::park();
    println!("resume child thread.");
  })
  .unwrap();
println!("spawn a thread");
thread::sleep(Duration::new(5, 0));
t.thread().unpark();
let _ = t.join();
println!("child thread finished");
```

## 免数据竞争

我们在闭包里面引用了函数体的局部变量，而这个闭包是运行在另一线程上，编译器无法肯定局部变量health的生命周期一定大于闭包的生命周期，于是发生了错误。

我们没有办法在多线程中直接读写普通的共享变量，除非使用Rust提供的线程安全相关的设施、

Data race 即数据竞争，意识是在多线程程序中，不同线程在没有使用同步的条件下并行访问同一块数据，且其中至少有一个是写操作的情况。

根据数据竞争的定义，它的发生需要三个条件：

- 数据共享，有多个线程同时访问一份数据
- 数据修改， 至少存在一个线程对数据做修改
- 没有同步，至少存在一个线程对数据的访问没有使用同步措施

只要让这三个条件无法同时发生即可：

- 可以禁止数据共享，比如actor-based concurrency , 多线程之间的通信紧靠发送消息来实现，而不是通过共享数据来实现。
- 可以禁止数据修改，比如function programming ，许多函数式编程语言严格限制了数据的可变性，而对共享性没有限制。

