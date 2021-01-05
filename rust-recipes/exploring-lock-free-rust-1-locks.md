# Exploring lock - free Rust 1: Locks

探索无锁Rust1：Lock

作为一个学习练习，我开始在Rust中实现一个简单的无锁算法。它的灵感来自于一个朋友工作的公司面试时提出的一个问题。这个问题很简单，初学者可以解决，但又很棘手，需要花点心思才能搞定--与原来的Java相比，Rust提出了一些新的挑战。

本系列文章介绍了一个简单的Rust无锁容器的演变过程，从单线程开始，到带锁的多线程变体，最后确定为无锁实现，并讨论了每一步的权衡。本书假设读者对Rust和多线程编程有基本的了解，但这些文章可能对这两方面的初学者有帮助。但请注意，作者不是无锁编程专家，所以可能会有错误--如果你发现一些错误，请留言。

## Contents

- The Exercise
- Rust API tradeoffs
- Compile-time thread safety
- Mutexes
- Fine-granied locking

## The Exercise

实现一个类，提供对缓存值的访问。更多细节

编写一个 LazyTransform 类，存储一个单一的值，允许它在需要时更新。这个值的计算有可能是昂贵的，所以setter方法，set_source 接受一个source，source将在lazyTransform的构造函数中使用接受的转换函数计算最终的值。

在被get_transformed请求之前，不得尝试变换。一旦生成，该值就会被缓存，并通过进一步调用get_transformed返回，直到被新的set_source调用变为无效。

单线程版本可以用下面的Python来总结：

```Python
class LazyTransform:
  def __init__(self, transform_fn): 
    self.transform_fn =  transform_fn
    self.source = None
    self.value = None
  
  def set_source(self, new_source):
    self.source = new_source
    
  def get_transformed(self):
    if self.source is not None:
      newval = self.transform_fn(self.source)
      if newval is not None:
        self.value = newval
        self.source = None
    return self.value
```

该类必须支持从多个线程调用，其语义如下:

- set_source和get_transformed可以并且将会在同一个LazyTransformer实例上并行调用。
- 一旦set_source完成，未来对get_transformed的调用最终必须开始返回新值。
- 预计读重的使用模式，所以无论set_source或get_transformed在其他线程中被调用多少次，get_transformed一定不会阻塞。唯一的例外是当检测到一个新的源时--允许get_transformed阻塞，直到转换完成后才返回转换后的值（并将其缓存以备将来调用）。
- 代码必须是无锁的：set_source和get_transformed都不应该卡住等待对方，即使它们被许多线程快速连续调用或并行调用，或者两者都调用。

## Rust API tradeoffs

在继续并行化之前，让我们回顾一下上述接口如何映射到Rust的类型系统。理想情况下，我们希望对源对象和最终对象使用的值的类型尽可能少的限制；可以是简单的单个u32，也可以是巨大的堆分配对象。例如，我们知道，源和值的类型都必须是Send，因为它们需要从不同于创建它们的线程访问。

另一个必要的限制是，最终值类型必须是Clone。为什么呢？考虑一下 "return of cached value "的概念，即上面Python中的return self.value一行，如何映射到Rust中。在Python中，语义很清楚，因为它的所有对象都是堆分配的，你总是能得到相同的实例共享。这也是最初的Java练习所指定的，它返回一个Object。但是一个正确的Rust实现需要处理一个存储在缓存中的实际值，并且有三个选项来返回它:

- 将对象从容器中移出，通常是将值字段设为Option，并返回self.value.take()。
- 返回对象的引用，返回&self.value。
- 克隆对象，并将克隆的值返回给调用者。

第一个选项显然是行不通的，因为它会阻止get_transformed返回缓存值一次以上。第二种方案看起来是可行的，直到我们考虑到返回的引用不能超过存储值的寿命。因为存储值可以被调用set_source而失效，而这种情况可以随时发生，很明显，允许返回一个引用是不健全的。事实上，所有这样的尝试都会被借用检查器立即拒绝.

虽然对于任意对象来说，克隆初看起来效率很低，但实际上它为用户提供了最大的灵活性。轻的值，比如数字ID是Copy（因此也是Clone）的，或者是克隆起来很便宜的小字符串，可以原封不动地放在缓存中。另一方面，重值可以作为Arc<ActualData>进行动态分配和访问，确保它们的克隆只增加一个引用计数，提供人们期望从等价的Python或Java中获得的语义。如果需要，甚至可以将两者结合起来，存储一个轻对象和一个重对象的元组。

所以，LazyTransform需要对值类型（T）和源类型（S）进行通用。但是我们不要忘记构造函数接收的变换函数。把它的类型固定为fn(S) -> T会限制它的无状态函数，而我们希望用户能够为变换提供一个任意的闭包。一种选择是在构造函数中接受一个通用的函数对象，并将其装箱在一个Box<Fn(S) -> T>中，但这将对每个LazyTransform实例施加一个动态分配，以及在调用函数时的一个间接性。如果转换函数在编译时是已知的，并且不携带任何状态，那么它应该既不产生存储也不产生运行时的间接开销。通过添加第三个类型参数，即变换函数的类型，可以很容易地实现这一点。

作为API的演示，这里是容器的单线程实现:

```rust
pub struct LazyTransform<T, S, FN> {
    transform_fn: FN,
    source: Option<S>,
    value: Option<T>,
}
impl<T: Clone, S, FN: Fn(S) -> Option<T>> LazyTransform<T, S, FN> {
    pub fn new(transform_fn: FN) -> LazyTransform<T, S, FN> {
        LazyTransform {
            transform_fn: transform_fn,
            source: None,
            value: None,
        }
    }
    pub fn set_source(&mut self, source: S) {
       self.source = Some(source);
    }
    pub fn get_transformed(&mut self) -> Option<T> {
        if let Some(source) = self.source.take() {
            let newval = (self.transform_fn)(source);
            if newval.is_some() {
                self.value = newval;
            }
        }
        self.value.clone()
    }
}
```

在精神上，这和原来的Python是完全一样的，只是加入了健康的静态输入法。

## Compile - time thread safety

如果我们试图在线程之间共享一个 LazyTransform 实例会发生什么？Rust会在编译时阻止这种情况发生--从多个线程中调用一个&mut方法将需要为同一个对象创建多个&mut引用，而这是被借用检查器阻止的。例如，以下内容无法编译。

```rust
pub mod lazy_transfrom;
fn main() {
    let fn_cal = |val: u32| Some(val + 1);
    let mut call = lazy_transfrom::LazyTransform::new(fn_cal);
    // println!("Hello, world!");
    std::thread::spawn(move || { // call moved here 
        for i in 0..10_000 { // value moved into closure here
            call.set_source(i); // variable moved due to use in closure
        }
    });

    while call.get_transformed().is_none() { //  call used after move 
        // value borrowed here after move
    }

    let val = call.get_transformed().unwrap();
    assert!(val >= 0 && val < 10_000);
}
```

他会被移动到由新线程执行的闭包中，但随后它就不能再被主线程使用了。通过引用发送它是行不通的，因为一个对象只能存在一个&mut引用，所以我们不允许将同一个引用发送给多个线程。动态分配LazyTransform并使用Arc在线程之间共享也没有用，因为Arc只提供对它拥有的数据的共享访问。

在Rust中，支持对容器的并行访问不仅需要改变实现，还需要改变方法签名。这是一个有意的设计决定--虽然Python或Java单线程代码在从多个线程调用时将愉快地执行，提供不正确的结果，但当从两个线程访问线程不安全的LazyTransform对象时，Rust版本将拒绝编译。

编译器使用简单的规则来决定线程之间共享对象是否安全:

- 从多个线程调用的方法必须接受&self而不是&mut self。这条规则也会被单线程代码的 borrow 检查器执行。

- 即使是通过共享引用，对象也不能包含专门为多线程访问而列入黑名单的类型的值。用 Rust 的术语来说，它的类型必须是 "Sync"，这意味着它实现了 Sync 标记特性，大多数对象都是这样。非Sync类型的例子是Rc或Cell，两者都有线程安全的等价物。

乍一看，第一条规则似乎排除了LazyTransform作为多线程类型的可能性。它的两个公有方法都清楚地修改了对象，其中set_source甚至以一种从外部可以观察到的方式进行修改。将签名改为接受&self而不是&mut self，编译失败，因为这两个方法都修改了&self共享引用后面的数据，这是被禁止的。访问&self内部的对象也会导致进一步的共享引用是只读的。

为了修改数据，我们必须找到一种方法，从对self的共享引用中获得一个独占的可变引用。这对于普通对象来说是不允许的，因为编译器将没有办法确保写入是排他性的，也就是说，当一个线程持有一个值的可变引用时，其他线程不能读取它或写入它。然而，如果我们能够静态地说服Rust，引用对数据的所有权将是独占的，那么它将在规则范围内允许转换。这就是mutexes的作用。

## Mutexes

Rust的Mutex类型提供了对其保护的值的读写访问，使用适当的操作系统基元来确保一次只能由一个线程完成。这里是LazyTransform的一个实现，更新为使用mutex。



