# 	A Persistent Singly - Linked Stack 

一种持久的单链栈



我们已经掌握了mutable singly  linked stacks

让我们通过编写一个持久不变的单链表，从单一所有权转移到共享所有权。

这正式函数式程序员所熟悉和喜欢的链表。

You can get the head or the tail and put someone's head on someone else's tail

基本上就是这样的，immutability is a hell of a drug.

在这个过程中，我们将很大程序上熟悉Rc和Arc。



添加一个名为third.rs的新文件

```rust
// in lib.rs

pub mod first;
pub mod second;
pub mod third;
```



## Layout



让我们回到layout上



关于持久列表最重要的一点是，你可以没有任何消耗的操作列表的尾部。

```
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D
```



但是我们最后希望内存中看起来是这样的

```
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+

```



这个在Box上行不通的。因为B的所有权时共享的。谁应该去释放它呢？如果我释放list2.他是否释放了B？对于Box,我们确定肯定是这样的。



函数式语言和几乎所欲其他语言都可以通过垃圾收集来解决这个问题。有了垃圾收集。B只有在所有人都停止查看之后才会被释放。



Rust没有这些语言所拥有的垃圾收集器。他们有tracing GC， 这将在运行时挖掘所有的内存，并自动找出那些是垃圾。

相反，Rust现今用到只是引用计数。引用计数可以被认为是一种非常简单的GC。对于许多工作负载。它的吞吐量比tracing GC

要低的很多。it completely falls over if you manager to build cycles. 

For our usecase we'll never run into cycles (fell ee to try to prove this to yourself )

那么，我们如何引入引用计数GC呢？ Rc就像Box一样。我们可以复制它，并且它的内存只有当所有的从Rc派生出的被删除时，内存才会被释放。也就是内存的引用计数变为0时才会被释放。不幸的是。这种灵活性带来了严重的代价，我们只能共享对象内部的引用。这意味着我们不能真正从我们的列表中获取数据，也不能对他们改变。



来看看我们的布局之前是这样的。

```rust
pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}
```



把它改为Rc试试。



```rust
// in third.rs

pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
  elem: T, 
  next: Link<T>,
}
```



```
cargo build

error[E0412]: cannot find type `Rc` in this scope
 --> src/third.rs:5:23
  |
5 | type Link<T> = Option<Rc<Node<T>>>;
  |                       ^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
1 | use std::rc::Rc;
  |
```

我们需要加上 use std::rc::Rc; 因为Rc不向Box那么通用。

改完之后OK可以编译过去。



## Basics 



我们现在已经知道了很多Rust的基础知识，所以我们可以再做很多简单的东西。



对于构造函数完全不需要变化

```rust
impl<T> List<T> {
  pub fn new() -> Self {
    Self {
      head: None,
    }
  }
}
```



push 和 pop已经没有意义了，相反，我们可以提供append 和 tail, 他们提供了大致相同的功能。

让我们从append开始。它接受一个list和一个elem。并返回一个List。与可变列表一种，我们希望创建一个新节点。它的下一个值是旧的列表。唯一新奇的事情是如何得到下一个值。因为我们不允许mutate anything .



答案就是Clone trait。 几乎所有的类型都实现了Clone trait.并提供了一种通用方法来获得“另一个像这样的类型”， 这个类型在逻辑上不相交，只是共享引用。还有不会被隐式地调用。



Rc特别适用Clone作为增加引用计数的方法。因此，与Box相比，将box奖移动到子列表中，我们只需要克隆旧列表的head。

我们甚至不需要match head, 因为Option 也公开了一个Clone实现。完全能实现我们需要的事情。



```rust
use std::rc::Rc;

pub struct List<T> {
  head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
  elem: T, 
  next: Link<T>,
}

impl<T> List<T> {
  pub fn new() -> Self {
    Self {
      head: None,
    }
  }
  
  pub fn append(&self, elem: T) -> List<T> {
    Self {
      head :  Some(Rc::new(Node {
        elem,
        next: self.head.clone(),
      }))
    }
	}
}

```



Ok!



Tail 是append相反的操作逻辑。它接受一个列表并返回第一个元素被删除的整个列表。这里需要做的就是Clone 列表中的第二个元素（如果存在的话）。

```rust
pub fn tail(&self) -> List<T> {
  Self {
    head: self.head.as_ref().map(|node| {
      node.next.clone()
    })
  }
}
```

```
cargo build

error[E0308]: mismatched types
  --> src/third.rs:27:22
   |
27 |         List { head: self.head.as_ref().map(|node| node.next.clone()) }
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::rc::Rc`, found enum `std::option::Option`
   |
   = note: expected type `std::option::Option<std::rc::Rc<_>>`
              found type `std::option::Option<std::option::Option<std::rc::Rc<_>>>`

```

Map期望我们返回返回一个y,但是这里我们返回一个Option < Y > 幸亏这是另一个常见的Option pattern, 我们可以使用and_then 来解决这个问题。

```rust
pub fn tail(&self) -> List<T> {
  List {
    head: self.head.ad_ref().and_then(|node| node.next.clone())
  }
}
```





```rust
pub fn map<U, F>(self, f: F) -> Option<U> 
where
	F: FnOnce(T) -> U,

// Maps an Option<T> to Option<U> by applying a fucntion to a contained value.

//example
//Converts an Option<String> into an Option<usize>, consummig the original:
let maybe_some_string = Some(String::from("hello, world"));
let maybe_some_len = maybe_some_string.map(|s| s.len());

assert_eq!(maybe_some_len, Some(13));

pub fn and_then<U, F>(self, f: F) -> Option<U> 
where
	F: FnOnce(T) -> Option<U>,

// returns None if the option is None, otherwise calls f with the wrapped value and return the result.
// Some languages call this operation flatmap

```



现在我们有了tail，我们可能应该提供head，它返回对第一元素的引用。

```rust
pub fn head(&self) -> Option<&T> {
  self.head.as_ref().map(|node| &node.elem )
}
```



测试

```rust
#[cfg(test)]
mod test {
  use super::List;
  
  #[test]
  fn basis(){
    let list = List::new();
    assert_eq!(list.head(), None);
    
    let list = list.append(1).append(2).append(3);
    assert_eq!(list.head(), Some(&3));
    
    let list = list.tail();
    assert_eq!(list.head(), Some(&2));
    
    let list = list.tail();
    assert_eq!(list.head(), Some(&1));
    
    let list = list.tail();
    assert_eq!(list.head(), None);
    
    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
  }
}
```

oK !



Iter is also identical to how it was for our mutable list:



```rust
pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<T> List<T> {
  pub fn iter(&self) -> Iter<'_, T> {
    Iter {
      next: self.head.as_ref().map(|node| &**node )
    }
  }
}

impl <'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| { ???
      self.next = node.next.as_ref().map(|node| &**node);
      &node.elem
    })
  }
}
```



```rust
#[test]
fn iter() {
  let list = List::new().append(1).append(2).append(3);
  
  let mut iter = list.iter();
  assert_eq!(iter.next(), Some(&3));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), Some(&1));
}
```



注意，我们不能为这种类型实现IntoIter或IterMut, 我们只能共享对元素的访问。



## Drop



与可变列表一样，我们也递归析构函数问题。不可否认，对于不可变列表来说，这个问题并不是那么糟糕：如果我们碰到另一节点，他是另一个列表的开头，我们不会递归地删除它。然而，这仍然是我们应该关心的事情。如何处理还不是很清楚。

下面这是以前的问题的解法

```rust
impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}
```



问题在于循环的主体

`cur_link = boxed_node.next.take();`



This is mutating the Node inside the Box, 但是我们不能做用Rc做到同Box一样的行为。他是给我们共享访问，因为任何其他Rc的数量都可以指向它。



但是如果我们知道这是这个列表的最后一个节点了。那么将Node移出Rc实际上是可以的。然后我们还知道什么时候停止：无论何时不能提升Node（whenever we can't hoist out the node)

Rc有一个方法就是这样的； try_unwrap



```rust
impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut head = self.head.take();
    while let Some(node) = head {
      if let Ok(mut node) = Rc::try_unwrap(node) {
        head = node.next.take();
      }else {
        break;
      }
    }
  }
}
```



```rust
pub fn try_unwrap(this: Self) -> Result<T, Self>

//Returns the inner value, if the Rc has exactly one strong reference.

//Otherwise, an Err is returned with the same Rc that was passed in.

//This will succeed even if there are outstanding weak references.

use std::rc::Rc;

let x = Rc::new(3);
assert_eq!(Rc::try_unwrap(x), Ok(3));
```


## Arc



使用不可变链表的一个原因是跨线程共享数据。毕竟，共享的可变状态是一切罪恶的根源，解决这个问题的一个方法就是永远消灭可变部分。



Except our list isn't thread-safe at all.  为了保证线程安全，我们需要原子地修改引用计数。否则，两个线程可以尝试增加引用计数，而只会发生一个。那么这个名单很快就会被释放出来。



为了保证线程的安全性。必须使用Arc。 Arc与Rc完全相同，只是引用计数是自动修改的。 

This has a bit of overhead if you don't need it , so Rust exposes both. 我们需要做的就是用std::sync::Arc 替换对Rc的所有引用。这样就是线程安全的了。



但是这提出了一个有缺的问题：我们如何知道一个类型是否是线程安全的？我们会不会不小心搞砸了？

在Rust中，你不能搞砸线程安全。



之所以会出现这种情况，是因为Rust以一种一流的方式对线程安全进行了建模，他有两个特性： Send和Sync



如果可以安全地移动到另一线程，则类型为Send.一个类型是Sync，如果他能在多线程之间安全的共享。

**也就是说如果T是Sync, &T就是Send. ** 在这种情况下，安全意味着不可能引起数据竞争（不要与更为一般的竞争条件问题搞混）



这是一个maker trait, 也就说他们是完全no interface. （Which is a fancy way of saying they're traits that provide absolutely not interface）你要么是Send, 或者不是，这只是其他API可能需要的一个属性。如果你没有适当的Send, 那么静态的就不可能发送到不同的线程（If you aren't appropriately Send, then it's staically impossible to be sent to a different thread!



Send和Sync 也是有根据你是否完全由Send和Sync组成而自动派生的特性。It's similar to how you can only implement Copy if you're only made of Copy types, but then we just go ahead and implement it automatically if you are.



几乎所有的类型都是Send和Sync。大多数类型是Send， 因为他们拥有自己的数据。**大多数类型都是Sync， 因为跨线程共享数据的唯一方法是将他们放在共享引用后面，这使得它们是不可变的。**

然而，也有一些特殊类型违反了这些属性：那些具有内部可变性的。到目前为止，我们只与继承的可变性进行了真正的交互：一个值的可变性是从其容器的可变性继承而来的。也就是说。你不能因为喜欢而随意变更不可变值的某个字段。



内部可变性类型违反了这一点：They let you mutate throught a shared reference. 

有两类主要的内部可变性:

Cell（只能在单线程上下文中工作）

Mutex(锁可以在多线程上下文中工作)。

显而易见的是。使用Cell更加chape。还有atomic，他们是原语，起着锁的作用。



那这一切与Rc和Arc有什么关系呢？他们都使用内部可变性作为引用计数。更糟糕的是，这个引用计数在每个实例之间是共享的。

Rc只使用Cell，这意味着他不是线程安全的。 Arc使用atomic这意味着他是线程安全的。当然，你不能通过把一个类型放在Arc中就神奇的使他变得安全。Arc只能像其他类型一样派生出线程安全的类型。（Arc can only derive thread-safety like any other type)



我真的不想深入了解原子内存模型或非派生的Send的实现。不用说。当你深入到Rust的线程安全。事情变得更加复杂。作为一个高层次的使用者。这一切是可行的。你真的不需要去考虑它。

