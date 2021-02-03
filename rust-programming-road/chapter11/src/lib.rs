/// # 第十一章 安全并发
/// 
/// ## 1.1 通用概念
/// 
///  
/// 并发就是同时应对多件事情的能力，并行就是同时执行多件事情的能力
/// 
/// 这也是两者之间的区别
/// 
/// 并发的概念
///    
/// 并发，关注点在于任务的切分，这是一种逻辑架构一种能力。
/// 
/// 并发就是单个个体可以处理多个不同的独立任务
/// 
/// 并行的概念
/// 
/// 并行，关注点在于同时执行，这是具体的实施状态。
/// 
/// 并行就是多个的单个单个个体处理各自的任务
/// 
/// 
/// 使用并发的主要两个原因： 性能和容错。
/// 
/// 在计算机中，通常使用一些独立的运行实体对并发进行支持，分为如下两类：
/// 
/// - 操作系统提供的进程和线程
/// - 编程语言内置的用户级线程
/// 
/// ## 11.1.1 多进程和多线程
/// 
/// 进程是资源分配的最小单元，线程时程序执行时的最小单元。
/// 
/// 从操作系统的角度来看，进程代表操作系统分配的内存、cpu时间片等资源的基本单位，他为程序提供基本的运行环境。 
/// 不同的应用程序可以按业务划分为不同的进程。
/// 从用户的角度来看，进程代表运行中的应用程序，他是动态条件下由操作系统维护的资源管理实体，而非静态的应用程序文件。
/// 每个进程都享有自己独立的内存单元，从而极大地提高了程序的运行效率。
/// 
/// 可以使用多进程来提高并发，比如Master-worker模式，由Master进程来管理Worker子进程，worker子进程执行任务
/// Master进程和worker进程之间通常使用Socket来进进程间通信(IPC)
/// 
/// 这样的好处就是具有极高的健壮性，当某个worker子进程出现问题时，不会影响到其他子进程。
/// 缺点，其中最让然诟病的是进程会占用相当可观的系统资源。还有，进程还有切换复杂，
/// CPU利用率低，创建和销毁复杂等缺点。
/// 
/// 
/// 为了需求比进程更小的资源占用，线程应运而生，线程时进程内的实体，它无法独立存在，
/// 必须依靠进程，线程的系统资源都来源于进程，包括内存。每个进程
/// 至少拥有一个线程，这个线程就是主线程。每个进程也可以生成若干个线程来并发执行多任务，但只能有一个
/// 主线程，线程和线程之间可以共享同一个进程内的资源，一个线程也可以创建或销毁另一个线程，
/// 所以线程会有创建，就绪，运行、阻塞和死亡五种状态。每个线程也有自己独享的资源，比如线程栈。
/// 线程和进程一样，都受操作系统内核的调度。线程拥有进程难以企及的优点，比如占用
/// 内存少，切换简单，CPU利用率高，创建、销毁简单，快速等。线程的缺点也是非常明显的，比如编程相当复杂，调试困难等。
/// 
/// 
/// 
#[test]
fn test() {

}