# 并发

## 线程

## 锁

## 队列

## 内存模型

## 后台线程

Background thread 只负责一件事，而且周期性醒来去做这件事。

## 通用线程池 worker pool

通过任务队列与客户端通信

## 管道 pipeline

将数据从一个线程导入另一个线程，每个线程只做一小部分工作。

## 数据并行 data parallelism

假设整个计算机主要用于一项大型计算，这个大型计算进而拆分成n个小任务，在n个线程上执行，希望所有n个机器的核心同时工作。

## 同步对象海

Sea of synchronized object 中多个线程拥有同一数据权限，使用基于互斥量等低级原语的临时锁方案避免争用，

## 原子整数操作

允许多和兴通过一个机器字大小的字段传递信息而实现通信。（除非要交换的数据就是整数值，否则这种方法比其他手段更难以保证正确，实践中，这通常意味着传递指针）

Rust为使用并发提供了更好的手段，但不是强迫所有程序员采用一种风格，而是安全地支持多种风格。无法言说的规则在代码中被写下来，由编译器负责监督。

## 使用Rust线程的3种方式

- 并行分叉-合并
- 通道
- 共享可修改状态

## 并行分叉- 合并的

## 优点

- 非常简单，分叉-合并很容易实现，在Rust中也很容易正确处理。
- 避免瓶颈，分叉-合并过程中不涉及给共享资源加锁。只有在任务结束后，一个线程才需要等待另一个线程。在执行任务期间，每个线程都可以自由运行。这样可以保证降低任务切换的开销。
- 性能计算直观。在最好的情况下，启动4个线程可以只用4分之一时间完成任务，不能对提速抱有理想的一个元婴，即可能无法为每个线层平均分派任务。另一个要注意的原因是，有时候分叉-合并在线程合并后必须花一些时间来组合所有线程计算的结果。换句话说，完全隔离任务可能造成额外的工作量，不过，除了这两点，任何CPU密集型程序在隔离工作单元下都会有显著的性能提升。
- 容易推断程序是否正确。只要线程之间是真正隔离的，分叉-合并程序就是确定性的。无论线程计算速度如何，程序最终都会得到相同的结果。这是一种无资源争用的并发模型。

## 缺点

要求工作单元隔离。
