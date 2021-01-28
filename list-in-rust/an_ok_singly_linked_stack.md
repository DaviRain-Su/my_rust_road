# An Ok Singly-Linked Stack 

一个可用的单链栈

在前面的章节中，我们写了一个最小的单链栈，然而，有一些设计决定使他有点糟糕。

让我们把它变得不那么糟糕

需要完善的

- Deinvent the whell 发明轮子
- Make out list able to handle any element type 是我们的列表能够处理任何元素类型
- Add peeking function
- Make out list iterable 使我们的列表可以迭代



因此需要学习的有

- Advanced Option use
- 泛型
- 生命周期
- 迭代器

添加一个名为second.rs的新文件：

```rust
//in lib.rs
pub mod first;
pub mod second;
```

然后把所有的东西从first.rs复制到里面。



## Use Option



对于 enum Link 我们相当于重新发明了一个非常糟糕的Option.

```rust
enum Link {
  Empty,
  More(Box<Node>),
}
```



Link 就是 Option<Box < Node >>,  因此`type Link = Option<Box<Node>>;`

现在，先用Option替换所有的东西,把Empty 改为None, More改为Some

```rust
use std::mem;

pub struct List{
  head: Link,
}

type Link = Option<Box<Node>>;

struct Node{
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    Self {
      head: None,
    }
  }
  
  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem,
      next: mem::replace(&mut self.head, None),
    });
    
    self.head = Some(new_node);
  }
  
  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, None) {
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
    let mut cur_link = mem::replace(&mut self.head, None);
    while let Some(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, None);
    }
  }
}
```



更急进一步的激进，可以将replace替换为Option中的take方法。

因为mem::replace(&mut Option, None) 是一个非常常见的惯用法，因此在Option中实际上就是take方法

```rust
pub struct List{
  head: Link,
}
type Link = Option<Box<Node>>;

struct Node {
  elem: i32,
  next: Link,
}

impl List {
  pub fn new() -> Self {
    Self {
      head: None,
    }
  }
  
  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });
    
    self.head = Some(new_node);
  }
  pub fn pop(&mut self) -> Option<i32> {
    match self.head.take() {
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
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}
```



再一次，`match option { None => None, Some(x) => Some(y) }`也是一种非常习惯的用法，因为被称为map.

map在Some(x)中的x上执行一个函数在Some(y)中生成y，因此可以编写一个函数传递给map，但是我们更愿意编写内联的操作。



因此要做到这一点，就是闭包。 闭包就是匿名函数，可以引用闭包之外的局部变量。这就使得闭包在执行各种条件逻辑时非常有用。

这里唯一需要改写就是pop方法

```rust
pub fn pop(&mut self) -> Option<i32> {
  self.head.take().map(|node| {
    self.head = node.next;
    node.elem
  })
}
```



## Making it all Generic 泛型



现在把代码中的具体地类型变成泛型

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



当然还要后面的方法实现也需要加上泛型，因为实现的结构中包含了泛型。跟在impl后面的是声明的泛型T，之后才可以使用。

```rust
impl<T> List<T> {
  pub fn new() -> Self {
    Self {
      head: None,
    }
  }
  
  pub fn push(&mut self, elem: T) {
    let new_node = Box::new(Node {
      elem,
      next: self.head.take(),
    });
    
    self.head = Some(new_node);
  }
  pub fn pop(&mut self) -> Option<T> {
    self.head.take().map(|node|{
      self.head = node.next;
      node.elem
    })
  }
}

impl <T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = self.head.take();
    while let Some(mut boxed_node) = cur_link {
      cur_link = boxed_node.next.take();
    }
  }
}

```



还有需要说明的一点

```rust
pub fn new() -> Self {
  Self {
    head: None,
  }
}
```

这里在new函数返回和函数体内构造时使用的都是Self这样，在后续的代码重构或者变更所做的改动很小，对于泛型也是，因为Self代表的就是你实现的那个类型



## Peek

我们还有一个方法没有做就是peek，这个方法返回的是（如果存在的话）列表头部元素的引用。



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



这里是因为map这个方法，会将Option进行移动。因为之前的pop方法我们需要的就是返回列表的头节点元素，之后就不再，但是这里我们还想让他继续留在那里。正确的处理方法是使用Optin的as_ref方法，



```rust
impl <T> Option<T> {
  pub fn as_ref(&self) -> Option<&T>;
}
```



It demotes the Option to an Option to a reference to its internals. 因此这个方法，作用在Option上返回的是一个用引用修饰内部元素的Option。 We could do this ourselves with an explicit match but ugh no. It does mean that we need to do an extra derdference to cut through the extra indirection, but thankfully the `.` Operator handles that for us.



```rust
pub fn peek(&self) -> Option<&T> {
  self.head.as_ref().map(|node| {
    &node.elem
  })
}
```



我们也可以通过as_mut创建一个可变的版本

```rust
pub fn peek_mut(&mut self) -> Option<&mut T> {
  self.head.as_mut().map(|ndoe| {
    &mut node.elem
  })
}
```



加入测试



```rust
#[test]
fn peek() {
  let mut list = List::new();
  assert_eq!(list.peek(), None);
  assert_eq!(list.peek_mut(), None);
  
  list.push(1);
  list.push(2);
  list.push(3);
  
  assert_eq!(list.peek_mut(), Some(&mut 2));
}
```



这里都没有问题，但是我们并没有真正的去测试是否可以改变这个peek_mut返回值。如果一个引用是可变的，但是没有去对它进行改变。那么我们这里就真的测试它的可变性了吗？

通过对Option<&mut T> 上使用map来测试



```rust
#[test]
fn peek() {
  let mut list = List::new();
  assert_eq!(list.peek(), None);
  assert_eq!(list.peek_mut(), None);
  
  list.push(1);
  list.push(2);
  list.push(3);
  list.peel_mut().map(|&mut value| {
    value = 42;
  });
  
  assert_eq!(list.peek(), Some(&42));
  assert_eq!(list.pop(), Some(42));
}
```



```
> cargo test

error[E0384]: cannot assign twice to immutable variable `value`
   --> src/second.rs:100:13
    |
99  |         list.peek_mut().map(|&mut value| {
    |                                   -----
    |                                   |
    |                                   first assignment to `value`
    |                                   help: make this binding mutable: `mut value`
100 |             value = 42
    |             ^^^^^^^^^^ cannot assign twice to immutable variable          ^~~~~

```



以这种方式编写的闭包的参数并不能指定这个值是一个可变引用。相反的是，它创建的模式将与闭包的参数匹配。

|&mut value| 的意识是，参数是一个可变引用，但是将他的值复制到value中，

如果指使用|value|那么value的类型就是&mut i32，这样就OK了。

这里给出的错误是value是不可变的，这是因为 map 在这里做的是模式匹配，map中的元素的类型是&mut T, 而这里给出的模式也是&mut value, 正好匹配到每一个，所以这里的value匹配到了T,这样就重新做出了一个新的绑定。

这样value就是不可变的了

```rust
#[test]
fn peek() {
  let mut list = List::new();
  assert_eq!(list.peek(), None);
  assert_eq!(list.peek_mut(), None);
  
  list.push(1);
  list.push(2);
  list.push(3);
  list.peel_mut().map(|value| {
    *value = 42;
  });
  
  assert_eq!(list.peek(), Some(&42));
  assert_eq!(list.pop(), Some(42));
}
```



## IntoIter



集合使用 Iterator trait在Rust迭代，这个比实现Drop要复杂点。

```rust
pub trait Iterator {
	type Item;
  fn next(&mut self) -> Option<Self::Item>;
}
```

type Item 是关联类型 ，这是在声明迭代器的每个实现都有一个名为Item的关联类型。

这样在调用next是 这个类型就是将要返回的类型。

Iterator 产生Option<  Self::Item > 的原因是因为接口合并了had_next和get_next的概念。

当你有下一个值是产生Some(value), 当你没有产生一个值是产生None. 这样使得API在使用和实现上更加符合人体工程学。更安全同时避免了冗余的检查和has_next和get_next之间的逻辑。

但是Rust还没有一个像yield statement的东西。 所以我们必须自己实现这个逻辑。

实际上集合应该实现三种不同的迭代器

- IntoIter - T
- IterMut - &mut T
- Iter - &T

实际上，我们已经有了所有的工具，可以使用List的接口实现。只需要一遍又一遍的调用pop，

因此，我们只需要实现作为List的新类型的包装器IntoIter (We'll just implement IntoIter as a newtype wrapper around List)



```rust
// Tuple strcut are an alternative form of struct
// useful for trival wrapper around other types

pub struct IntoIter<T>(List<T>);

impl <T> List<T> {
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    // access fields of tuple struct numerically
    self.0.pop()
  }
}
```



测试



```rust
#[test] 
fn into_iter() {
  let mut list = List::new();
  list.push(1);
  list.push(2);
  list.push(3);
  
  let mut iter = list.into_iter();
  assert_eq!(iter.next(), Some(3));
  assert_eq!(iter.next(), Some(2));
  assert_eq!(iter.next(), Some(1));
  assert_eq!(iter.next(), None);
}
```



## Iter



这次不能依靠List提功能的接口来实现了。 我们需要的基本逻辑时保持一个指针，指向我们下一个要生成的当前节点。因为这个节点可能不存在（对空list进行迭代或者已经迭代完）所以我们希望引用是Option，当我们产生一个元素时，我们希望继续到当前节点的下一个节点。



```rust
pub struct Iter<T> {
  next: Option<&Node<T>>,
}

impl <T> List<T> {
  pub fn iter(&self) -> Iter<T> {
    Iter {
      next: self.head.map(|node| &node)
    }
  }
}
impl <T> Iterator for Iter<T> {
    type Item = &T;
    
    fn next(&mut self) -> Option<Self::Item> {
      self.next.map(|node| {
        self.next = node.next.map(|node| &node);
        &node.elem
      })
    }
  }
}
```



```
> cargo build

error[E0106]: missing lifetime specifier
  --> src/second.rs:72:18
   |
72 |     next: Option<&Node<T>>,
   |                  ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
  --> src/second.rs:82:17
   |
82 |     type Item = &T;
   |                 ^ expected lifetime parameter
```



缺少生命周期



让我们查看这个错误，rustc可以给出帮助。

```
> rustc --explain E0106
This error indicates that a lifetime is missing from a type. If it is an error
inside a function signature, the problem may be with failing to adhere to the
lifetime elision rules (see below).

Here are some simple examples of where you'll run into this error:

struct Foo { x: &bool }        // error
struct Foo<'a> { x: &'a bool } // correct

enum Bar { A(u8), B(&bool), }        // error
enum Bar<'a> { A(u8), B(&'a bool), } // correct

type MyStr = &str;        // error
type MyStr<'a> = &'a str; //correct
...

```



说的不是很清楚，让我们将`'a` 加入到结构中试试

```rust
pub struct Iter<'a, T> {
	next: Option<&'a Node<T>>,
}
```



```
> cargo build

error[E0106]: missing lifetime specifier
  --> src/second.rs:83:22
   |
83 | impl<T> Iterator for Iter<T> {
   |                      ^^^^^^^ expected lifetime parameter

error[E0106]: missing lifetime specifier
  --> src/second.rs:84:17
   |
84 |     type Item = &T;
   |                 ^ expected lifetime parameter

error: aborting due to 2 previous errors
```



让我们继续在方法的实现中添加

```rust
pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> List<T> {
  pub fn iter(&'a self) -> Iter<'a, T>  {
    Iter{ next: self.head.map(|node| &'a node) }
  }
}

impl <'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&'a mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.map(|node| &'a node);
      &'a node.elem
    })
  }
}
```



```
> cargo build

error: expected `:`, found `node`
  --> src/second.rs:77:47
   |
77 |         Iter { next: self.head.map(|node| &'a node) }
   |         ---- while parsing this struct        ^^^^ expected `:`

error: expected `:`, found `node`
  --> src/second.rs:85:50
   |
85 |             self.next = node.next.map(|node| &'a node);
   |                                                  ^^^^ expected `:`

error[E0063]: missing field `next` in initializer of `second::Iter<'_, _>`
  --> src/second.rs:77:9
   |
77 |         Iter { next: self.head.map(|node| &'a node) }
   |         ^^^^ missing `next`
```



我们需要弄清楚这里的生命周期'a 是什么、



生命周期在垃圾回收的语言中，是不必要的，因为垃圾收集器可以保证所有东西可以奇迹般的生存早它需要的时间。

Rus中的大多数的数据都是需要手动管理的，因此数据需要另一种结局的方案。

C和C++可以给我们一个清晰的例子，如果只是让人们获取堆栈上的随机数据的指针，会发行说呢个什么呢？无处不在的无法管理的安全。

这个可以大致的分为两类错误

- 拿着一个超出范围的东西的指针
- 拿着一个指针，指向一个变化的东西

生命周期就是解决这两个问题，他们用一种完全的透明的方式来做这件事。



那么什么是lifetime?

声明周期是程序中某个地方的代码区域（block/scope)的名称。当引用被标记为lifetime时，我们说他必须对整个区域有效。

Different things palce requirements on how long a reference must and can be valid for. 

整个lifetime系统返过来又是一个试图最小化每个引用的区域的约束求解系统。如果他成功地找到了一组满足所有约束的lifetime，则编译通过。否则你就会得到一个错误的回复，说什么东西没有获得足够长。



在一个函数体重，通常不能谈论lifetime，也不想去谈论。编译器拥有完整的信息。可以推断出所有的约束以找到最小的lifetime。然而，在类型和API-level，编译器并不具备所有的信息。它要求你告诉它不同lifetime之间的关系。这样编译器就能知道你在做什么了。



原则上，这些生命周期可以省略的，但是检查所有的borrow将是一个庞大的整个程序分析，将产生另一难以置信的非本地错误。Rust系统意味着所有的兼用检查都可以在每个函数体重独立完成，而且所有的错误应该是相当局部的（或者你的类型有不正确的签名）



Rust也提供了生命周期的省略规则：



```rust
//Only one reference in input, so the output must be derived from that input
fn foo(&A) -> &B;
fn foo<'a>(&'a A) -> &'a B;

// Many inputs, assume they're all independent
fn foo(&A, &B, &C); 
fn foo<'a, 'b, 'c>(&'a A, &'b B, &' c C);

// Method, assume all outout lifetime are derived from self
fn foo(&self, &B, &C) -> &D;
fn foo<'a, 'b, 'c>(&'a self, &b B, &'c C) -> &'a D;
```



`fn foo<'a>(&'a A) -> &'a B`是什么意思呢？ 意味着输入的存货时间至少与输出的时间一样长。因此，如果长时间保持输出，这将扩展输入必须也是有效的区域。一旦停止输出，编译器就知道输入也可以无效。



通过这个系统的建立，Rust可以保证在free之后没有任何东西呗使用，(and nothing is mutated while outstanding references exist). 它只是确保所有的约束都能解决。



这里先回到没有lifetime的状态

```rust
pub struct Iter<T> {
  next: Option<&Node<T>>,
}

impl <T> List<T> {
  pub fn iter(&self) -> Iter<T> {
    Ifer { next: slef.head.map(|node| & node )}
  }
  
}

impl <T> Iteraor for Iter<T> {
  type Item = &T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.map(|node| &node);
      &node.elem
    })
  }
}
```



我们只需要在函数和签名中添加lifetime



```rust
//Iter is generis over some lifetime, it doesn't care
pub struct Iter<'a , T> {
  next: Option<&'a Node<T>>,
}

// No lifetime here, List doesn't have any associated lifetimes
impl <T> List<T> {
  // W declare a fresh lifetime here for the exact borrow that
  // create the iter. Now &self needs to be valid as long as the Iter is around.
  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Ifer { next: slef.head.map(|node| &node )}
  }
  
}

// We do have a lifetime here, because Iter has on that we need to define
impl <'a, T> Iteraor for Iter<'a, T> {
  //Need it here too, this is a type declaration
  type Item = &'a T;
  
  // None of this needs to changes, handled but the above
  //self contiues to be incredibly hype and amazing
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      //self.next = Option<Somethin>
      //重新赋一个新的值，它的下一个值的不可变引用
      self.next = node.next.map(|node| &node);
      &node.elem
    })
  }
}
```



```
cargo build

error[E0308]: mismatched types
  --> src/second.rs:77:22
   |
77 |         Iter { next: self.head.map(|node| &node) }
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `second::Node`, found struct `std::boxed::Box`
   |
   = note: expected type `std::option::Option<&second::Node<T>>`
              found type `std::option::Option<&std::boxed::Box<second::Node<T>>>`

error[E0308]: mismatched types
  --> src/second.rs:85:25
   |
85 |             self.next = node.next.map(|node| &node);
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `second::Node`, found struct `std::boxed::Box`
   |
   = note: expected type `std::option::Option<&'a second::Node<T>>`
              found type `std::option::Option<&std::boxed::Box<second::Node<T>>>`

```



我们想要存储的是&Node 's, 但是我们得到的是&Box < Node >'s, 这里只需要在我们获取reference之前dereference the Box.



```rust
// No lifetime here, List doesn't have any associated lifetimes
impl <T> List<T> {
  // W declare a fresh lifetime here for the exact borrow that
  // create the iter. Now &self needs to be valid as long as the Iter is around.
  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Ifer { next: slef.head.map(|node| &*node )}
  }
  
}

// We do have a lifetime here, because Iter has on that we need to define
impl <'a, T> Iteraor for Iter<'a, T> {
  //Need it here too, this is a type declaration
  type Item = &'a T;
  
  // None of this needs to changes, handled but the above
  //self contiues to be incredibly hype and amazing
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.map(|node| &*node);
      &node.elem
    })
  }
}
```



```
cargo build
   Compiling lists v0.1.0 (/Users/ABeingessner/dev/temp/lists)
error[E0515]: cannot return reference to local data `*node`
  --> src/second.rs:77:43
   |
77 |         Iter { next: self.head.map(|node| &*node) }
   |                                           ^^^^^^ returns a reference to data owned by the current function

error[E0507]: cannot move out of borrowed content
  --> src/second.rs:77:22
   |
77 |         Iter { next: self.head.map(|node| &*node) }
   |                      ^^^^^^^^^ cannot move out of borrowed content

error[E0515]: cannot return reference to local data `*node`
  --> src/second.rs:85:46
   |
85 |             self.next = node.next.map(|node| &*node);
   |                                              ^^^^^^ returns a reference to data owned by the current function

error[E0507]: cannot move out of borrowed content
  --> src/second.rs:85:25
   |
85 |             self.next = node.next.map(|node| &*node);
   |                         ^^^^^^^^^ cannot move out of borrowed content

```



这里忘记使用as_ref，因为Box是所有权的，Box会被移动到map中，这意味着他将会被删除，这意味着我们的引用会悬空。

```rust
pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl <T> List<T> {
  pub fn iter<'a>(&'a self) -> Iter<'a,T> {
    Ifer { next: slef.head.as_ref().map(|node| &*node )}
  }
  
}

impl <'a, T> Iteraor for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &*node);
      &node.elem
    })
  }
}
```



```
cargo build
   Compiling lists v0.1.0 (/Users/ABeingessner/dev/temp/lists)
error[E0308]: mismatched types
  --> src/second.rs:77:22
   |
77 |         Iter { next: self.head.as_ref().map(|node| &*node) }
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `second::Node`, found struct `std::boxed::Box`
   |
   = note: expected type `std::option::Option<&second::Node<T>>`
              found type `std::option::Option<&std::boxed::Box<second::Node<T>>>`

error[E0308]: mismatched types
  --> src/second.rs:85:25
   |
85 |             self.next = node.next.as_ref().map(|node| &*node);
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `second::Node`, found struct `std::boxed::Box`
   |
   = note: expected type `std::option::Option<&'a second::Node<T>>`
              found type `std::option::Option<&std::boxed::Box<second::Node<T>>>`


```



as_ref添加了一层引用，我们需要移出的这一层。

```rust
pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

impl <T> List<T> {
  pub fn iter<'a>(&'a self) -> Iter<'a,T> {
    Ifer { next: slef.head.as_ref().map(|node| &**node )}
  }
  
}

impl <'a, T> Iteraor for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      &node.elem
    })
  }
}
```



你可能会说这个 &** 真的难看，但是通常Rust擅长隐式地做这种转换，通过一个deref的进程执行，基本上可以在整个代码中插入 *'s 来进行类型检查。它可以做到这一点，因为我们有借入检查器，确保我们从来没有搞乱指针。



但是在这种情况下，结合我们有一个Option<&T>而不是&T的事实。所以需要这样做。

还有一种不同的方法turbofish:

```rust
self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
```



map是一个通用的函数

`pubfn map<U, F>(self, f: F) -> Option<U>`

Turbofish `::<>` 让我们告诉编译器我们认为这些泛型的类型应该是什么，在这种情况下`::<&Node<T>, _>`表示他应该返回 `&Node<T>`, 我们不需要关系其他类型。



这返过来让编译器知道&node应该对他应用deferf coercion ，所以我们不需要手动引用所有那些*'s .



编写测试

```rust
#[test]
fn iter() {
  let mut list = List::new();
  list.push(1);
  list.push(2);
  list.push(3);
  
  let mut iter = list.iter();
  
  assert_eq!(iter.next(), Some(&3));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), Some(&1));
}
```



还要一个，lifetime可以省略

```rust
impl<T> List<T> {
  pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter { next: self.head.as_ref().map(|node| &**node )}
  }
}
```



等价于

```rust
impl<T> List<T> {
  pub fn iter(& self) -> Iter<T> {
    Iter { next: self.head.as_ref().map(|node| &**node )}
  }
}
```



如果不喜欢隐藏某个结构的lifetime可以这样做, rust 2018显式省略生命周期语法



```rust
impl<T> List<T> {
  pub fn iter(& self) -> Iter<'_, T> {
    Iter { next: self.head.as_ref().map(|node| &**node )}
  }
}
```



## IterMut



从语义上来讲，the nature of shared and mutable references means that iter is 'trivial' while IterMut is Legit Wizard Magic.



关键的洞察力来自于我们对Iter的Iterator实现

```rust
impl <'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  
  fn next(&mut self) -> Option<Self::Item> {
    //....
  }
}
```



因此可以是这样的

```rust
impl <'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  
  fn next<'b>(&'b mut self) -> Option<Self::Item> {
    //....
  }
}
```



next的签名在输入和输出之间不建立任何约束，所以我们为什么要去关注呢？



```rust
let mut list = List::new();
list.push(1);
list.push(2);
list.push(3);

let mut iter = list.iter();
let x = iter.next().unwrap();
let y = iter.next().unwrap();
let z = iter.next().unwrap();

```



这对于共享引用来说绝对是好事，因为关键在于你可以同时拥有大量的引用。然而，可变引用不能共存。关键在于他们是排外的。



最终的结果是，使用安全代码编写IterMut显得更加困难。但是IterMut实际上可以实现许多结构。

先从Iter代码开始，把所有的东西改成可变的。

```rust
pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
  pub f iter_mut(&mut self) -> IterMut<'_, T> {
    IterMut{ next: self.head.as_mut().map(|node| &mut **node)}
  }
}

impl <'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a T;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_mut().map(|node| &mut **node);
      &mut node.elem
    })
  }
}
```



```
> cargo build
error[E0596]: cannot borrow `self.head` as mutable, as it is behind a `&` reference
  --> src/second.rs:95:25
   |
94 |     pub fn iter_mut(&self) -> IterMut<'_, T> {
   |                     ----- help: consider changing this to be a mutable reference: `&mut self`
95 |         IterMut { next: self.head.as_mut().map(|node| &mut **node) }
   |                         ^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error[E0507]: cannot move out of borrowed content
   --> src/second.rs:103:9
    |
103 |         self.next.map(|node| {
    |         ^^^^^^^^^ cannot move out of borrowed content

```



这里的第一个错误上面已经改过来了，因为要的可变的版本。`pub fn iter_mut(&self) -> IterMut<'_, T>`这样就是错误的。



对于第二个。



对于之前不是可变的版本，使用的是默认是复制的语义。当引入所有权时，当你移动东西的时候，你就不能再使用它了。对于某些类型来说，这完全是合理的。Box就是为我们管理堆上的数据，我们当然不希望两端代码认为他们需要释放内存。



对于基本类型来说，这是无用的。因为他们默认是复制语义的。是按位复制的。当移动时。旧的值仍然可以用。

As a consequence, you can even move a Copy out of reference without replacement!



Rust中的所有数字类型都是复制语义。也可以将任何用户定义类型声明为Copy，只要是其所有组件是Copy的。

关键是为什么这个代码可以工作，因为共享引用也是Copy的。因为&是Copy,所以Option <& >也是Copy的。 因此当我们做`self.next.map`的时候，他是OK的，因为Option被复制了。现在这样就不行了。因为&mut 不是Copy的。（如果你复制了&mut，那么就会有两个&mut 得内存的一个位置，这在Rust中的借用检查是禁止的）。因袭我们需要使用Option的take方法来获取。



```rust
fn next(&mut self) -> Option<Self::Item> {
  self.next.take().map(|node| {
    //take操作之后self.next = None
    //后面复制一个新的值
    self.next = node.next.as_mut().map(|node| &mut **node);
    &mut node.elem
  })
}
```



测试

```rust
#[test]
fn iter_mut() {
  let mut list = List::new();
  list.push(1);
  list.push(2);
  list.push(3);
  
  let mut iter = list.iter_mut();
  assert_eq!(iter.next(), Some(&mut 3));
  assert_eq!(iter.next(), Some(&mut 2));
  assert_eq!(iter.next(), Some(&mut 1));
}
```



还有一些需要说的是：



我们实现的这个代码，接受单链表，并最多一次返回链表中的每个元素的可变引用。他是静态验证的。而且绝对安全。不需要做任何事情。



这样的原因是因为：

- We take Option<&mut > so we have exclusive access to the mutable reference . No need to wory about someone looking at it again. (我们take的Option<&mut>，所以我们拥有唯一的访问权限，不用担心有人再次查看)
- Rust understands that it's ok to shared a mutable reference into subfields of the pointed-to struct, because there's no way to "go back up", and they're definitely disjoint.(Rust理解将一个可变的引用切分到 point-to struct 的子字段是可以的，因为没有方法返回，而且他们肯定是不相交的)

事实证明，可以应用这个基本逻辑为数组或树获得安全的IterMut.甚至可以将Iterator设置为doubleEnded,这样就可以同时从front和back使用迭代器。





