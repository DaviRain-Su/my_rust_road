# 总结

## A Bad Singly-Linked Stack 

### Basic Data Layout

什么是链表呢？它是在堆上的一堆数据按顺序指向对方。

一个链表的定义：（函数式语言中给出的定义）

`List a = Empty | Elem a (List a) `

a List is either Empty or an Element followed a List. This is **Sum Type**.

在Rust中:

```rust
enum List {
    //^^^^ recursive type has infinite size
    Empty,
    Elem(i32, List), //这是一个递归的定义
    //recursive without indirection
}
```
可以看到的是这个是一个indinite size Tyep，
Elem(i32, List), 中包含了有一个List,而第一个List的大小需要清楚的知道到底有多少个List,才能确定他的大小。

Rust给出的解决提示：

**insert indirection (eg, Box, Rx, &) at some point to make**

```rust
#[lang = "owned_box"]
pub struct Box<T>(_)
 where
    T: ?Sized;
//A pointer type for heap allocation.

// pub fn new(x: T) -> Box<T>

//Allocates memory on the heap and then places x into it.

//This doesn't actually allocate if T is zero-sized.

let five = Box::new(5);

// Box<T>, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
```

因此链表的定义是：
```rust
enum List{
    Empty,
    Elem(i32, Box<List>),
} 
```

第一种链表的布局：

```
[] = Stack 
() = Heap 

[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)

```

1、最后一个节点Empty中什么数据也没有他也需要占用内存，因为这个节点后续可能变为一个有数据的节点，因此，这个空闲的Empty也需要占用一个数据的大小和一个指针的内存大小。

2、第一个节点没有进行堆内存分配

Enum 的内存布局

![Kitten]( ./pic/1.png "Enum Type")

第二种链表的布局：

```
[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
```

在这个布局中我们可以随意的分配堆节点。
这里的不同之处(和第一个布局的不同之处在于，最后一个节点的 junk 数据)

```rust
enum Foo {
    D1(T1),
    D2(T2),
    D3(T3),
}
```

Foo 中需要存储一些整数来指示它表示的枚举变量D1, D2, D3,这个就是枚举的标签Tag,可以看上面的内存布局图就是上面的Tag，也需要占用内存。还需要存储足够的空间来存储T1,T2, T3这里也是就是上面内存布局图中的 E中的内存布局分布大小。此外还有一些额外的空间需要满足对齐的要求。


对于布局1， 尽管Empty是一个单独信息位， 但它必须为一个指针和一个元素消耗足够的空间，因为他随时准备好成为一个Elem。

因此第一布局堆分配了一个额外的元素，该元素充满了垃圾，比布局2要消耗更多的空间。

这里布局1的第一个节点没有分配，但是总是别分配要要糟糕的，因为这会带来非统一的布局。

这个对于push和pop没有什么影响，但是对于链表的分隔会带来问题：

```
layout 1:

[Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)

split off C:

[Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
[Elem C, ptr] -> (Empty *junk*)


```

```
layout 2:

[ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)

split off C:

[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
[ptr] -> (Elem C, *null*)

```
可以看到的是布局2的拆分仅仅复制B中指向堆内存的指针就可以了，对于布局1来说还必须将C也从堆内存复制到栈内存中去。

链表的好处是，可以在节点本身中构造元素，然后在不移动它的情况下在列表中移动它。

这样做也是不对的：

```rust
enum List {
    Empty, 
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List>),
}
```

这个是糟糕的，因为有一个完全无效的状态，ElemThenNotEmpty(0, Box< Empty >).

这个完全避免了分配Empty case, 将堆分配的总数减少了1，但是这会浪费更多的空间。（这是因为前面布局利用了空指针优化）。


### 空指针优化
> 原文作者：Rust 技术论坛文档：《Rust 高级编程（2018）》
转自链接：https://learnku.com/docs/nomicon/2018/110-ffi/4756


一些 Rust 类型被定义为永不为 null，包括引用（&T、&mut T）、Box<T>、以及函数指针（extern "abi" fn()）。可是在使用 C 的接口时，指针是经常可能为 null 的。看起来似乎需要用到 transmute 或者非安全代码来处理各种混乱的类型转换。但是，Rust 其实提供了另外的方法。

一些特殊情况中，enum 很适合做空指针优化，只要它包含两个变量，其中一个不包含数据，而另外一个包含一个非空类型的成员。这样就不需要额外的空间做判断了：给那个包含非空成员的变量传递一个 null，用它来表示另外那个空的变量。这种行为虽然被叫做 “优化”，但是和其他的优化不同，它只适用于合适的类型。

最常见的受益于空指针优化的类型是 Option<T>，其中 None 可以用 null 表示。所以 Option<extern "C" fn(c_int) - > c_int> 就很适合表示一个使用 C ABI 的可为空的函数指针（对应于 C 的 int (*)(int)）。

```rust
extern crate libc;
use libc::c_int;

extern "C" {
    // 注册回调。
    fn register(cb: Option<extern "C" fn(Option<extern "C" fn(c_int) -> c_int>, c_int) -> c_int>);
}

// 这个函数其实没什么实际的用处。它从C代码接受一个函数指针和一个整数，
// 用整数做参数调用指针指向的函数，并返回函数的返回值。
// 如果没有指定函数，那默认就返回整数的平方。
extern "C" fn apply(process: Option<extern "C" fn(c_int) -> c_int>, int: c_int) -> c_int {
    match process {
        Some(f) => f(int),
        None    => int * int
    }
}

fn main() {
    unsafe {
        register(Some(apply));
    }
}
```

一个简单的例子：空指针优化

```rust
enum Foo {
    A, 
    B(i32),
}
```

空指针起作用，这样啊就能消除tag所占用的空间。
如果变量是A， 则整个枚举都设置为0， 否则变量是B。
这里的B不可能是0，因为他包含了一个非零指针。

```rust
fn main(){
    println!("Foo = {}", std::mem::size_of::<Foo>()); // 8
}

enum Foo { // tag 4
    A, // 0
    B(i32),// 4
}
```
其他一些类型和Enum配合使用能使用空指针优化的是：
&, &mut, Box, Rc, Arc, Vec.

如何才能同时实现避免额外的内存分配， 统一的内存分配， 空指针优化？

我们需要区分拥有一个元素和分配一个列表的概念。
Enum可以把包含多个不同类型的值中的一个，struct允许同时声明一个包含多个值的类型。

> Enum是和类型，struct是积类型。

List要么是Empty，或者是一个元素后跟着一个List。

```rust
struct Node {
    elem: i32,
    next: List,
}

pub enum List { //pub enum 会将字段全部公开
    Empty,
    More(Box<Node>),// error, Node is Private
}


```

```rust
pub struct List {
    head: Link, //List只有一个字段，所以他的大小和该字段相同
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
```

## NEW

New方法也可以称为Rust中的构造函数方法。

```rust
impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }
}
```
上面两处的Self的使用的优点，后面如果有代码的变动，例如结构的名字的变动，后期代码的重构。
函数的最后一个表达式是隐式返回的。
`::是命名空间操作符`

## 所有权

所有权的三种不同的语义：

- self - Vale, Value具有移动语义，具有所有权
- &mut self - mutable reference，可变引用（唯一引用）， 对值没有所有权，只是对不拥有的得值临时独占访问权（这也是称为唯一引用的原因）， 可以进行任何mutable 操作
- &self - shared reference，共享引用，对不拥有的值临时的共享访问，不允许进行mutable操作。


## PUSH

```rust
impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head, //error, cannot move out of borroed content
        };
    }
}
// push中的self是&mut self类型，只是可变借用了slef。
//对于结构体的初始化，next,是具有移动语义类型的成员，next: self.head 这样会发生移动操作，但是rust中明确的规定了可变引用没有拥有所有权，所以出错。

```

如果把东西在在放回去呢？

```rust
pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
        elem: elem,
        next: self.head, // error, cannot 
    });
    // 我们在结束后放回去，应该是可以的但是rust中不可以。

    
    self.head = Link::More(new_node);
}
```

std::mem::replce(&mut dst, src);

This incredibly useful function lets us steal a value out of a borrow by replacing it with another value.

可以通过一个替换操作mem::replce()， 让我们通过用另一个值替换一个借用来窃取一个值。

![Kitten]( ./pic/indy.gif "Enum Type")

使用replace 之后：

```rust

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}
```

## POP

pop要返回一个值，这个值可能有（list不为空时）或者为空，这样返回一个Option<i32>就可以了。

```rust

pub fn pop(&mut self) -> Option<i32> {
    match self.head {
        Link::Empty => {
            // TODO
        }
        Link::More(node) => {
            // TODO
        }
    };
}

```


使用unimplemented!()， 表示还没有完成函数的实现

```rust
pub fn pop(&mut self) -> Option<i32> {
    match self.head {
        Link::Empty => {
            // TODO
        }
        Link::More(node) => {
            // TODO
        }
    };
    unimplemented!()
}

```

```
> cargo build

error[E0507]: cannot move out of borrowed content
  --> src/first.rs:28:15
   |
28 |         match self.head {
   |               ^^^^^^^^^
   |               |
   |               cannot move out of borrowed content
   |               help: consider borrowing here: `&self.head`
...
32 |             Link::More(node) => {
   |                        ---- data moved here
   |
note: move occurs because `node` has type `std::boxed::Box<first::Node>`, which does not implement the `Copy` trait
  --> src/first.rs:32:24
   |
32 |             Link::More(node) => {
   |                        ^^^^

```

默认情况下，match尝试将其内存移动到新的分支。

通过在self.head上加一个reference解决。

```RUST
pub fn pop(&mut self) -> Option<i32> {
    match &self.head {
        Link::Empty => {
            // TODO
        }
        Link::More(ref node) => {
            // TODO
        }
    };
    unimplemented!()
}

```

```RUST
pub fn pop(&mut self) -> Option<i32> {
    let result;
    match &self.head {
        Link::Empty => {
            result = None;
        }
        Link::More(ref node) => {
            result = Some(node.elem);
            self.head = node.next; //ERROR, NODE 是一个reference, Cannot move out of borroed content
        }
    };
    result
}

```
```
> cargo build
   Compiling lists v0.1.0 (/Users/ABeingessner/dev/temp/lists)
error[E0507]: cannot move out of borrowed content
  --> src/first.rs:35:29
   |
35 |                 self.head = node.next;
   |                             ^^^^^^^^^ cannot move out of borrowed content


```

- Check if the list is empty.

- If it's empty, just return None
    - If it's not empty
    - remove the head of the list
    - remove its elem
    - replace the list's head with its next
    - return Some(elem)

这里是因为，要删除东西，意味着通过所有权（vale）获取list的head。不能通过&self.head来操作。

这里只有一个可变引用， 所以只能通过replace来实现了


```rust
pub fn pop(&mut self) -> Option<i32> {
    let result;
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => {
            result = None;
        }
        Link::More(node) => {
            result = Some(node.elem);
            self.head = node.next;
        }
    };
    result
}
```

优化，一个函数的最后一个表达式就是返回的值。

```RUST
pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => None,
        Link::More(node) => {
            self.head = node.next;
            Some(node.elem)
        }
    }
}
```

## TEST

```rust
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

```

## DROP

Rust中的析构函数等同就是Drop trait来实现的：

```RUST
pub trait Drop {
    fn drop(&mut self);
}
```

对于list来说， drop的自动析构可能会有问题。
在list中，你要做的就是drop list的head， 而head又会drop Box< Node >, 

考虑：
list -> A -> B -> C 

当list被删除时， 它尝试删除A， 而A尝试删除B， 而B尝试删除C，这是一个递归的代码。

```rust
impl Drop for List {
    fn drop(&mut self) {
        // NOTE: you can't actually explicitly call `drop` in real Rust code;
        // we're pretending to be the compiler!
        self.head.drop(); // tail recursive - good!
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {} // Done!
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // tail recursive - good!
            }
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // uh oh, not tail recursive!
        deallocate(self.ptr);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

```

我们没有办法删除Box后面的内容，所以没有以尾递归的方式删除。
不得不编写一个迭代的版本删除节点。

```RUST
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

```


## FINAL CODE

```RUST

#![allow(unused_variables)]
fn main() {
use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
}
```

# An Ok Singly-Linked Stack 

- 使用Option
- 加入泛型
- 加入迭代器

```rust
use std::mem;

pub struct List {
    head: Link,
}

//Update 
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None } //update
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),// update
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None, // update
            Some(node) => { //update 
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None); //update
        while let Some(mut boxed_node) = cur_link { // update 
            cur_link = mem::replace(&mut boxed_node.next, None); // update
        }
    }
}

```

mem::replace(&mut option, None), 在Option中是很常见的习惯用法，所以Option提供了一个等同的方法take()

```rust
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() { // update take way
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();//update take way
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();// update take way
        }
    }
}

```

对于
```rust
match Option {
    None => None, 
    Some(x) => {
        // do something
        Some(y)
    }
}

这是一个非常常用的用法，等价于map()这个函数， 使用方法将一个闭包函数传递给map()
```

```rust 
//before
pub fn pop(&mut self) -> Option<i32> {
    match self.head.take() { // update take way
        None => None,
        Some(node) => {
            self.head = node.next;
            Some(node.elem)
        }
    }
}

//after 
pub fn pop(&mut self) -> Option<i32> {
    self.head.take().map(|node| {
        self.head = node.next;
        node.elem
    })
}
```

### Generic 

加入泛型

```rust
pub struct List {
    head: Link<T>,
}

type Link<T> = Option<Box<Node>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() { // update take way
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();//update take way
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();// update take way
        }
    }
}
```

## PEEK

```rust
pub fn peek(&self) -> Option<&T> {
    self.head.map(|node| {
        &node.elem
    })
}

```

```
> cargo build

error[E0515]: cannot return reference to local data `node.elem`
  --> src/second.rs:37:13
   |
37 |             &node.elem
   |             ^^^^^^^^^^ returns a reference to data owned by the current function

error[E0507]: cannot move out of borrowed content
  --> src/second.rs:36:9
   |
36 |         self.head.map(|node| {
   |         ^^^^^^^^^ cannot move out of borrowed content



```

这是因为self.head是Option< Box < Node < T > >>是一个具有移动语义的类型，使用map的话默认是获取的所有权，但是peek的方法是一个不可变引用， cannot move out of borrowed content，如果把这个借用中的所有权移走后，这个借用就悬空了，rust的借用规则禁止这样做。

可以使用Option中的as_ref方法返回的是引用

```rust
impl<T> Option<T> {
    pub fn as_ref(&self) -> Option<&T>;
}
```
引用是Copy语义的，所以map方法不会将所有权转移走。

```RUST

// 不可变版本
pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| {
        &node.elem
    })
}

// 可变版本
pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| {
        &mut node.elem
    })
}

```

test

```rust
#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| {
        *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}


```

## IntoIter

迭代器

```rust
pub trait Iterator {
    type Item; //关联类型,这个是下次调用后返回的类型。
    fn next(&mut self) -> Option<Self::Item>;
}

```

迭代器的返回值使用Option< Self::Item > 原因是因为融合了get_next()和has_next()，的两个概念。

rust提供了三种不同的迭代器：

- IntoIter - T
- IntoMut - &mut T
- Iter - &T

使用list的pop实现

```rust

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

```

```rust

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

```

## Iter 

Iter - &T 

Iter中保存的是一个指针，指向下一个要生成的当前节点。因为生成的节点可能不存在所以使用Option包装。

```rust

pub struct Iter<T> {
    next: Option<&Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.map(|node| &node) } //error 
    }
}

impl<T> Iterator for Iter<T> {
    type Item = &T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.map(|node| &node); //error
            &node.elem
        })
    }
}

```

因为其中使用的是引用，所以需要使用生命周期参数。上面编译会出错。

生命周期参数是为编译器准备的，声明周期的参数使用只有一个原则，就是输出的生命周期不能大于输入的生命周期。（即， 借用方不能大于出借方的生命周期）


一些生命周期的省略规则

```rust
// Only one reference in input, so the output must be derived from that input
fn foo(&A) -> &B; // sugar for:
fn foo<'a>(&'a A) -> &'a B;

// Many inputs, assume they're all independent
fn foo(&A, &B, &C); // sugar for:
fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

// Methods, assume all output lifetimes are derived from `self`
fn foo(&self, &B, &C) -> &D; // sugar for:
fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;

```


一个OK的版本

```Rust


pub struct Iter<'a, T> { //加入了lifetime, 结构体中使用时必须先声明
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    //加入了lifetime,函数中使用必须先声明
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) } // 使用as_ref是因为map默认方式是移动语义
        // 这里的两个** 是因为as_ref或多一个， Box也是一个，所以需要两个**， 解引用

        // 另一种写法
        // Iter {     self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
        //     }
    }
}

// impl<T> List<T> {
//     pub fn iter(&self) -> Iter<T> {
//         Iter { next: self.head.as_ref().map(|node| &**node) }
//     }
// }


// impl<T> List<T> {
//     pub fn iter(&self) -> Iter<'_, T> {
//         Iter { next: self.head.as_ref().map(|node| &**node) }
//     }
// }


// 加入了lifetime, 
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            
            self.next = node.next.as_ref().map(|node| &**node); // 这里的原因同上
            &node.elem
        })
    }
}

```

```rust
#[test]
fn iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

```


## IterMut 

**IterMut - &mut T**


```rust
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| { //因为&mut self是唯一引用，不能复制，不能存在多个这里需要使用take(),来保证，不会存在多个可变引用
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

```

前面说过map会按移动语义来捕获变量， 但是as_ref, 返回的是不可变引用，对于不可变引用是Copy 语义，所有权语义这里也不会出现问题。

对于Copy的类型，移动之后原来的值还在。所有as_ref的是使用就OOK了。

但是对于&mut T 的引用是不能同时同时存在多个的，所以不具有Copy trait， 


```rust
#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}

```


## Final Code 

```rust

#![allow(unused_variables)]
fn main() {
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}

}
```



# A Persistent Singly - Linked Stack 

从单一所有权list转移到共享所有权list


将要使用Rc， Arc.

## Layout 

Persistent List 可以方便的操作list


下面的操作方式是很少见的:

```
list1 = A -> B -> C -> D
list2 = tail(list1) = B -> C -> D
list3 = push(list2, X) = X -> B -> C -> D

list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+

```

这里对于使用Box存储数据的list是不行的，因为Box是具有所有权属性的。
这里就会出现问题谁去应该释放它？而且， 这样的一个方式，在Box中是不可能会出现的。


对于函数式语言中这是可能的， 拥有垃圾回收的语言中都回去实现这样的功能。

B只有在所有的人都停止查看之后才会被释放。

Rc就是简单的具有引用计数的智能指针， 就像Box一样，但是我们可以去复制它，并且它的内存只有当所有Rc的派生被删除出时，内存才会被释放。

但是，Rc的缺点是： 不能真正的获取内部的值，不能可变其内部中包装的值。只用去共享的使用内存的引用。

```rust
use std::rc::Rc; 

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


## BASIC

```rust
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
}
```

append方式是接受一个list和一个元素，返回一个list。
创建一个新的节点，它的下一个值是旧列表。这里的特点是如何得到下一个值，因为这里我们不允许变化任何东西。

通过的及时Rc的clone 特性。Rc的Clone会增加引用计数的方法， 
相当与使用Rc的clone来复制所有权，与将Box移动到子列表中，我们只需要克隆旧列表的头部。甚至不用去匹配head，因为Option中也提供了clone的方法。

```rust
pub fn append(&self, elem: T) -> List<T> {
    List { head: Some(Rc::new(Node {
        elem: elem,
        next: self.head.clone(),
    }))}
}

pub fn tail(&self) -> List<T> {
    List { head: self.head.as_ref().map(|node| node.next.clone()) }
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

这里是因为map期望返回的是Y(std::rc::Rc)， 但是这里返回的是Option(), 这是可以使用Option中的额and_then()

```rust
pub fn and_then<U, F>(self, f: F) -> Option<U>
where
    F: FnOnce(T) -> Option<U>,
```

```rust
pub fn tail(&self) -> List<T> {
    List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
}

```
head方法返回第一个元素的引用

```rust
pub fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.elem )
}
```

```rust
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
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


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}
#[test]
fn iter() {
    let list = List::new().append(1).append(2).append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

```

我们不能为这种类型实现 IntoIter 或 IterMut，我们只能共享对元素的访问

## DROP

```rust
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take(); 
            // 这里有问题的是，Rc中会有多个节点指向的问题，只有当这个节点是最后一个没有引用计数指向时，可以删除
        }
    }
}

//通过Rc的try_unwrap()

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

```
## ARC

不可变意味着可以跨线程共享，共享可变的状态是罪恶的。


Arc, 线程安全的引用计数

rust中通过Send, Sync来标记一个类型是线程安全的。
如果可以安全的移动到另一个线程，则这个类型是Send的。
如果可以安全的在线程之间共享， 这个类型是Sync的。

如果T是Sync的，则&T是Send的。这样线程安全意味着捕获引起数据竞争。

Send和Sync也是dreived的根据这个结构中的类型是否都实现了Send,Sync.来实现。

内部可变性的Cell（不是线程安全的）和RefCell（线程安全的）常常和Rc（单线程）, Arc（多线程）配合使用。


## FINAL CODE

```RUST

#![allow(unused_variables)]
fn main() {
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
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
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

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
}

```


# An  Unsafe Singly-Linked Queue

通常使用Refcell会使事情变得很复杂，

这里将会使用原生指针和unsafe rust.

Stack的操作：

因为这里是单链表实现Queue， 对于Stack来说，FIFO的的特性只在一端操作就可以了，但是对于Queue来说，如果是由单链表来实现，Queue的特性是LIFO，所以pop 和 push操作是在两个不同的方向操作的。因此Queue的push和pop当有一端在另一侧是，需要去遍历整个list，在效率上很低。


```
input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

stack push X:
[Some(ptr)] -> (X, Some(ptr)) -> (A, Some(ptr)) -> (B, None)

stack pop:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

```
Queue的操作:

```
input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

flipped push X:
[Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

input list:
[Some(ptr)] -> (A, Some(ptr)) -> (B, Some(ptr)) -> (X, None)

flipped pop:
[Some(ptr)] -> (A, Some(ptr)) -> (B, None)

```

因此在结构体中是不是可以存放一个引用或者指针去保存一个尾节点的指针呢？

这样是可以的，但是对于Rust来说，存放引用的话，会因为在push或者pop时给自身的结构体中保存了一份可变的引用，这样就会违反借用规则。这样是不可以的。

```rust
pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>, // NEW!
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            // When you push onto the tail, your next is always None
            next: None,
        });

        // Put the box in the right place, and then grab a reference to its Node
        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                // If the old tail existed, update it to point to the new tail
                old_tail.next = Some(new_tail);
                old_tail.next.as_mut().map(|node| &mut **node)
            }
            None => {
                // Otherwise, update the head to point to it
                self.head = Some(new_tail);
                self.head.as_mut().map(|node| &mut **node)
            }
        };

        self.tail = new_tail;
    }
    pub fn pop(&'a mut self) -> Option<T> {
        // Grab the list's current head
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // If we're out of `head`, make sure to set the tail to `None`.
            if self.head.is_none() {
                self.tail = None;
            }

            head.elem
        })
    }

}

mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}


```

```
cargo test

error[E0499]: cannot borrow `list` as mutable more than once at a time
  --> src/fifth.rs:68:9
   |
65 |         assert_eq!(list.pop(), None);
   |                    ---- first mutable borrow occurs here
...
68 |         list.push(1);
   |         ^^^^
   |         |
   |         second mutable borrow occurs here
   |         first borrow later used here

error[E0499]: cannot borrow `list` as mutable more than once at a time
  --> src/fifth.rs:69:9
   |
65 |         assert_eq!(list.pop(), None);
   |                    ---- first mutable borrow occurs here
...
69 |         list.push(2);
   |         ^^^^
   |         |
   |         second mutable borrow occurs here
   |         first borrow later used here

error[E0499]: cannot borrow `list` as mutable more than once at a time
  --> src/fifth.rs:70:9
   |
65 |         assert_eq!(list.pop(), None);
   |                    ---- first mutable borrow occurs here
...
70 |         list.push(3);
   |         ^^^^
   |         |
   |         second mutable borrow occurs here
   |         first borrow later used here


....

** WAY MORE LINES OF ERRORS **

....

error: aborting due to 11 previous errors

```

可以看到的问题出现了，当push时出现了多个可变的借用，这个在rust中是违反了借用规则的。



所以解决的方式是，存放一个裸指针（原生指针）保存指向尾节点。

```rust
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}
```

## UNSAFE

Rust中原生指针有两种： *const T, *mut T, 对应的就是C中的const T*, T*;

Unsafe 中的一些特性：

## BASIC


构造函数发生了变化，因为tail中存放的是原生指针，这里初始化使用的是ptr::null_mut()。

```rust
use std::ptr;

// defns...

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }
}

```

这里push的，结构体中tail中存放的是一个原生指针。


`let raw_tail: *mut _ = &mut *new_tail;
`
引用变为指针操作。

```rust
pub fn push(&mut self, elem: T) {
    let mut new_tail = Box::new(Node {
        elem: elem,
        next: None,
    });

    let raw_tail: *mut _ = &mut *new_tail;

    // Put the box in the right place, and then grab a reference to its Node
    if !self.tail.is_null() {
        // If the old tail existed, update it to point to the new tail
        unsafe {
            // 对于raw pointer操作是不安全的操作，必须放在unsafe 中，通过unsafe 表明这部分操作是不安全的操作
            (*self.tail).next = Some(new_tail);
            // 使用raw pointer， 必须手动defer，因为这是一个不安全的操作， 这里还涉及到了运算符的优先级问题
        }
    } else {
        // Otherwise, update the head to point to it
        self.head = Some(new_tail);
    }

    self.tail = raw_tail;
}

pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|head| {
        let head = *head;
        self.head = head.next;

        if self.head.is_none() {
            // head为None, 将taill设置为null
            self.tail = ptr::null_mut();
        }

        head.elem
    })
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}

```

对于迭代是完全可以复用过来的

```rust
// ...

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}




impl<T> List<T> {
    // ...

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}





#[cfg(test)]
mod test {
    // ...

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}


```

## FINAL CODE 

```rust
#![allow(unused_variables)]
fn main() {
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
}
```


# A Bad but Safe Doubly - Linked Deque

通过Rc和 RefCell的内部可变性实现一个双向链表

RefCell中的核心方法：

```rust
fn borrow(&self) -> Ref<'_, T>;
fn borrow_mut(&self) -> RefMut<'_, T>;
```

可以将borrow和borrow_mut的借用规则和&T, &mut T的规则是一样的，只不过RefCell是在运行时做借用检查。出现问题是会直接panic!

由于实现的是双向链表，因此每一个节点中都有一个指向上一个节点和下一个节点的指针。

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

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                //这里使用了borrow_mut(),因为Rc中存放的是RefCell<Node<T>>
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}


```

RefCell是在运行时做借用检查的，
RefCell是可共享的可变容器


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




## FINAL CODE


```rust

#![allow(unused_variables)]
fn main() {
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

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


impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

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
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
}
```





# The Double Single 

对于双向链表，因为rust拥有严格的所有权，所以没有一个节点严格的拥有其他节点。

现在把一个链表一份为2，一部分在左边，一部分在右边。

每一边是一个栈，这样列表可以向左或者向右增加，方法是压栈操作。我可以遍历这个list，从一段弹出值到另一端。

```rust

pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    // pub fn push(&mut self, elem: T) {
    //     let new_node = Box::new(Node {
    //         elem: elem,
    //         next: self.head.take(),
    //     });

    //     self.head = Some(new_node);
    // }
    pub fn push(&mut self, elem: T){
        let new_node = Box::new(Node {
            elem,
            next: None,
        });

        self.push_node(new_node);
    }

    pub fn push_node(&mut self, mut node : Box<Node<T>>){
        node.next = self.head.take();
        self.head = Some(node);
    }

    // pub fn pop(&mut self) -> Option<T> {
    //     self.head.take().map(|node| {
    //         let node = *node;
    //         self.head = node.next;
    //         node.elem
    //     })
    // }

    pub fn pop(&mut self) -> Option<T> {
        self.pop_node().map(|node|{
            node.elem
        })
    }

    pub fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.take().map(|mut node |{
            self.head = node.next.take();
            node
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


pub struct List<T> {
    left: Stack<T>,
    right: Stack<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            left: Stack::new(), 
            right: Stack::new(),
        }
    }
    
    pub fn push_left(&mut self, elem: T){
        self.left.push(elem)
    }
    pub fn push_right(&mut self, elem: T){
        self.right.push(elem)
    }
    pub fn pop_left(&mut self) -> Option<T> {
        self.left.pop()
    }
    pub fn pop_right(&mut self) -> Option<T> {
        self.right.pop()
    }
    pub fn peek_left(&mut self) -> Option<&T> {
        self.left.peek()
    }
    pub fn peek_right(&mut self) -> Option<&T> {
        self.right.peek()
    }
    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        self.left.peek_mut()
    }
    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.right.peek_mut()
    }

    pub fn go_left(&mut self) -> bool {
        self.left.pop_node().map(|node|{
            self.right.push_node(node);
        }).is_some()
    }

    pun fn go_right(&mut slef) -> bool {
        self.rigth.pop_node().map(|node|{
            self.left.push_node(node);
        }).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn walk_aboot() {
        let mut list = List::new();             // [_]

        list.push_left(0);                      // [0,_]
        list.push_right(1);                     // [0, _, 1]
        assert_eq!(list.peek_left(), Some(&0));
        assert_eq!(list.peek_right(), Some(&1));

        list.push_left(2);                      // [0, 2, _, 1]
        list.push_left(3);                      // [0, 2, 3, _, 1]
        list.push_right(4);                     // [0, 2, 3, _, 4, 1]

        while list.go_left() {}                 // [_, 0, 2, 3, 4, 1]

        assert_eq!(list.pop_left(), None);
        assert_eq!(list.pop_right(), Some(0));  // [_, 2, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(2));  // [_, 3, 4, 1]

        list.push_left(5);                      // [5, _, 3, 4, 1]
        assert_eq!(list.pop_right(), Some(3));  // [5, _, 4, 1]
        assert_eq!(list.pop_left(), Some(5));   // [_, 4, 1]
        assert_eq!(list.pop_right(), Some(4));  // [_, 1]
        assert_eq!(list.pop_right(), Some(1));  // [_]

        assert_eq!(list.pop_right(), None);
        assert_eq!(list.pop_left(), None);

    }
}
```




