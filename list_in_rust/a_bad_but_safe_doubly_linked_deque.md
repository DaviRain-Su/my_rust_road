# A Bad but Safe Doubly Linked Deque

这里开始关注 Rc和内部可变性问题。通过内部可变性可以改变Rc内部的值。
这里的内部可变性将借用检查放到了运行时去做检查。

如果可以这样做的话，就可以实现一个完全的双向链表。

在这个过程中，我们将熟悉内部可变性。这可能是学的最艰难的方式。安全并不意味着正确。
双向链表很难，我们总是在某个地方犯错。

```
// in lib.rs
pub mod first;
pub mod second;
```

这一章基本上是一个示范，这是一个非常坏的主意

## Layout 

我们设计的关键是RefCell类型，RefCell的核心是一对方法：

``
fn borrow(&self) -> Ref<'_, T>;
fn borrow_mut(&self) -> RefMut<'_, T>;
``

borrow 和 borrow_mut的规则和&，&mut的借用规则是一样的。因此borrow
你可以借用多次，而borrow_mut有排他性。。

RefCell的借用规则不是静态执行的，而是在运行时去执行的。如果你违反了借用规则，
RefCell就会panic并且crash 这个程序。

为什么这里要返回Ref和RefMut？

他们的行为基本上就像Rc只不过是借用的。
They also keep the RefCell borrowed until they they go out of scope.

现在有了Rc和RefCell。 (An incredibly verbose pervasively mutable garbage collected languange 
 that can't collect cycles!)
 
 我们需要doubly linked,这意味着每个节点都有一个指向上一个和下一个节点的指针。并且list本身还有一个指向第一个节点和
 最后一个节点的指针。这样能使我们在list的两端快速的插入和删除

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}
```

## Building Up

先从建立链表开始。

```rust
impl <T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new( Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }
}
```

现在开始写push, 双向链表要很复杂，我们需要做更多的工作。在单链表操作可以简化为单线程的情况下，双向链表操作要相当复杂。

特别是，我们需要特别处理一些与空list有关的边界问题。大多数的操作只会触及到head和tail。但是当转换到空的list时或者从list转换到一个不空的list是，
我们需要同时操作head和tail。

要验证我们的方法是否有意义，一个简单的方法是我们保持以下的invariant:

每个节点应该有两个指向他的指针。在列表中的每个节点都有前置节点和后续节点指向他。而末端的节点则由列表本省指向。

```rust
use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl <T> Node<T> {
     fn new(elem: T) -> Rc<RefCell<Self>> {
         Rc::new(RefCell::new( Node {
             elem,
             prev: None,
             next: None,
         }))
     }
}
 
impl <T> List<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn push_front(&mut self, elen: T) { 
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non - empty list, need to conect the old_head
                old_head.prev = Some(new_head.clone()); // +1 new_head
                new_head.next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                // total : +2 new_head, +0 old_head --- ok!
            },
            None => {
                // empty list, need to set 
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
                //total: +2 new_head -- ok!
            }   
            
        }   
}

}
```


```
cargo build

error[E0609]: no field `prev` on type `std::rc::Rc<std::cell::RefCell<fourth::Node<T>>>`
  --> src/fourth.rs:39:26
   |
39 |                 old_head.prev = Some(new_head.clone()); // +1 new_head
   |                          ^^^^ unknown field

error[E0609]: no field `next` on type `std::rc::Rc<std::cell::RefCell<fourth::Node<T>>>`
  --> src/fourth.rs:40:26
   |
40 |                 new_head.next = Some(old_head);         // +1 old_head
   |                          ^^^^ unknown field

```

为什么我们不能访问节点的prev和next字段？以前我们只有一个Rc<Node>的时候,他是OK的。似乎是RefCell在阻止我们。

RefCell 

具有动态检查借用规则的可变内存location (A mutable memory location with dynamically checked borrow rules)


可共享的可变容器

Cell<T> 和 RefCell<T> 类型的值可以通过共享（即 &T type) mutate, 然而大多数rust类型只能通过unique reference (&mut) mutate。 
我们认为Cell<T>和RefCell<T>提供了内部可变性，而typical Rust type 表现出 inherited mutability.

Cell type 有两种： Cell<T>, RefCell<T>,
Cell<T> 提供了get,set方法通过单个方法调用改变内部的值。 Cell<T> 只能与实现Copy trait兼容。对于其他类型，必须使用RefCell<T>， 在mutating之前获得a wirite lock 

RefCell<T> uses Rust's lifetimes to implement 'dynamic borrowing ', a process whereby one can claim temporary exclusive, mutable access to
the inner value.

RefCell<T> 的借用在运行时被追踪，不像Rust的native reference 是在整个编译期被追踪的。因为RefCell<T>的借用是动态，所以可以尝试应可变借用已经借用的值。
当这种情况发生时会导致thread panic 

https://stackoverflow.com/questions/47915905/do-a-containers-members-inherit-its-mutability
When to choose interior mutability， Where one must have unique access to mutate a value, is one of the key language 
elements that enable Rust to reason strongly about pointer aliasing, statically preventing crash bugs.

因此，Inherited mutability 是首选，而内部可变性是最后的手段。 Since cell types enable mutation where it would otherwise be 
disallowed though, there are occasions when interior mutability might be appropriate, or even must be used.
- Introducing inherited mutability root to shared types. 向共享类型引入 inherited mutability root
- Implementation details of logically-immutable methods 逻辑不可变方法的实现细节
- mutating implementations of Clone

Introducing inherited mutability roots to shared types

共享智能指针类型包括Rc<T>, Arc<T>， provide containers that can be cloned and shared between multiple parties. 
Because the contained values may be multiply-aliased, they can only borrowed as shared references, not mutable reference.
Without it would be impossible to mutate data inside of shared boxed at all.

在共享指针雷子那个中加入RefCell<T>来重新引入可变是很常见的。

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
}
```
这里使用的是Rc<T>而不是Arc<T>, RefCell<T> 用于单线程方案，如果在多线程环境中需要共享可变性，需要使用Mutex<T>

`shared_map.borrow_mut().insert('africa', 92388);`

所以可以这样实现

```rust
impl <T> List<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None }
    }

    pub fn push_front(&mut self, elen: T) { 
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non - empty list, need to conect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head); // +1 old_head
                self.head = Some(new_head); // +1 new_head, -1 old_head
                // total : +2 new_head, +0 old_head --- ok!
            },
            None => {
                // empty list, need to set 
                self.tail = Some(new_head.clone()); // +1 new_head
                self.head = Some(new_head); // +1 new_head
                //total: +2 new_head -- ok!
            }   
            
        }   
}

}
```

## Breaking Down

pop_front 应该和push_front的基本逻辑是相同的，但是backwards

```rust
impl <T> List<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring it's -2
        self.head.take().map(|old_head| { // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => { // -1 new
                    // not emptying list 
                    new_head.borrow_mut().prev.take(); // -1 old
                    self.head = Some(new_head); // +1 new
                    // total: -2 old, +0 new
                },
                None => {
                    // emptyhing list
                    self.tail.take(); // -1 old
                    // total: -2 old, +0 new
                }   
            }   
            old_head.elem
        })
    }

}
```

```
> cargo build

error[E0609]: no field `elem` on type `std::rc::Rc<std::cell::RefCell<fourth::Node<T>>>`
  --> src/fourth.rs:64:22
   |
64 |             old_head.elem
   |                      ^^^^ unknown field

```

通过borrow_mut 操作下

```rust

pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|old_head| {
        match old_head.borrow_mut().next.take() {
            Some(new_head) => {
                new_head.borrow_mut().prev.take();
                self.head = Some(new_head);
            }
            None => {
                self.tail.take();
            }
        }
        old_head.borrow_mut().elem
    })
}

```

```
cargo build

error[E0507]: cannot move out of borrowed content
  --> src/fourth.rs:64:13
   |
64 |             old_head.borrow_mut().elem
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content

```

borrow_mut只能给我们一个&mut Node<T>, 但是we can't move out of that!

我们需要一个可以接受RefCell<T>并且可以给我们一个T的东西

`fn into_inner(self) -> T`

consumes the RefCell, return the wrapped value

old_head.into_inner().elem

```
> cargo build

error[E0507]: cannot move out of an `Rc`
  --> src/fourth.rs:64:13
   |
64 |             old_head.into_inner().elem
   |             ^^^^^^^^ cannot move out of an `Rc`

```

我们想要移出RefCell， 但是我们不能因为他在一个Rc中， 
正如我们在前一章看到的，Rc<T>指允许我们将共享引用引入到他的内部。 that's the whole point of the reference counted pointed : They're shared!
Rc<T> only lets us get shared references into its internals.

This was a problem for us when we wanted to implement Drop for our reference counted list, and the solution is the same:
 Rc::try_unwrap, which moves out the contents of an Rc if its refcount is 1.
 
 Rc::try_unwrap(old_head).unwrap().into_inner().elem
 
 Rc::try_unwrap 将返回 Result<T, Rc<T>> , Result are basically a generalized Option, where None case has data associated with it
 在种种情况下，当你试图unwrap时，由于我们不关系它的失败情况。我们就只调用unwrap、
 
 ```
> cargo build

error[E0599]: no method named `unwrap` found for type `std::result::Result<std::cell::RefCell<fourth::Node<T>>, std::rc::Rc<std::cell::RefCell<fourth::Node<T>>>>` in the current scope
  --> src/fourth.rs:64:38
   |
64 |             Rc::try_unwrap(old_head).unwrap().into_inner().elem
   |                                      ^^^^^^
   |
   = note: the method `unwrap` exists but the following trait bounds were not satisfied:
           `std::rc::Rc<std::cell::RefCell<fourth::Node<T>>> : std::fmt::Debug`

```

unwrap On Result 需要你 debug-print error case, RefCell<T> only implements Debug if T does. 
Node doesn't implement Debug.

不如让我们通过将Reuslt变成一个Option的OK

Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem

```rust

#[cfg(test)]
mod test{
    use super::List;
    
    #[test]
    fn basic() {
        let mut list = List::new();
        
        // check empty list behaves right
        assert_eq!(list.pop_front(), None);
        
        // populate list 
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        // check normal removal
        
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        
        //push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);
        
        //check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
    
        //check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }   
}
```
现在我们可以正确地从列表中删除一些东西，我们可以实现Drop, 这次Drop在概念上更有缺一些，以前我们为了避免无线递归而为Drop， 现在我们需要实现爱内Drop
来实现任何事情。

Rc不能处理循环，如果我们有一个循环，所有的东西都活保持alive, 事实证明， 一个双向链表实际一个由小循环组成的大链。 当我们删除我们的链表时，
两个末端节点refcount decremented down to 1，然后什么也不会发生。如果我们的列表只包含一个节点，我就可以开始了。但是理想情况下，如果列表包含多个元素，那么他就应该
正常工作。

正如我们所看到的，删除元素有点痛苦。所以对我们来说最简单的事情就是pop，知道我们得到None

```rust
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
```
