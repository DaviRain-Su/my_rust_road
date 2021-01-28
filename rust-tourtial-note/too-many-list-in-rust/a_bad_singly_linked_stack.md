# Learn Rust with Entirely Too Many Linked Lists

**How to implement a linked list in Rust**

Should Learn: 
- The following pointer types: `&, &mut, Box, Rc, Arc, *const, *mut`
- `Ownership, borrowing, inherited mutability, interior mutability, Copy`
- All The Keywords: `struct, enum, fn, pub, impl, use, ...`
- `Pattern matching, generics, destructors`
- Testing
- Basic Unsafe Rust

LISTS:

1. A Bad Singly-Linked Stack 
2. An OK Singly-Linked Stack
3. A Persistent Singly-Linked Stack
4. A Bad But Safe Doubly-Linked Deque
5. An Unsafe Singly-Linked Queue
6. TODO: An OK Unsafe Doubly-Linked Deque
7. Bonus: A Bunch of Sily Lists

Use Cargo to Manager our projects:

$ cargo new --lib lists

$ cd lists

Put each list in a seperate file so that we don't lose any of our work.

Authentic Rust Learning experience involves writing code , having the compiler scream at you.

Learning to read and Understand Rust's generally excellent compliler errors and documentation is incredibly important to being a productive Rust programmer.

Several great use cases for a linked list:
- you want to do a lot of splitting or merging of big lists
- you're doing some awesome lock-free concurrent thing.
- you're using a pure function language and the limited semantics and absence of mutation make linked lists easier to work with.
- and more ...

But all of these cases are super rare for anyone writing a Rust program program. 99% of the time you should just use a Vec(array stack), and 99% of the other 1% of the time you should be using a VecDeque(array deque). These are blatantly superior data structures for most workloads due to less frequent allocation, lower memory overhead, true random access, and cacher locality.

Linked lists are as niche and vague of a data structure as a tirs. Few would blak at me claiming a trie is a niche structure that you average programmer could happily never learn in an entire productive career -- and yet linked lists have some bizarre status. We teach wvery undergrad how to write a linked list. It's the only niche collection I couldn't kill from std::collections. It's the list in C++! 

RUST LINKEDLIST SOURCE CODE: https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html

# A Bad Singly-Linked Stack （一个坏的单链接 堆栈）


This one's gonna be by far the longest, as we need to introduce basically all of Rust, and are build up some things "the hard way" to better understand the language.

这将是迄今为止最长的，因为我们需要介绍基本上所有的Rust， 并要建立一些东西"艰难的方式", 来更好的理解语言。

We'll put our first list in src/first.rs. We need to tell Rust that first.rs is something that our lib uses. All that requires is that we put this at the top of src/lib.rs(which Cargo made for us):

我们将将把第一个列表放在src/first中。我们需要告诉Rust，first.rs是我们的库使用的东西。 
所需要做的就是把它挡在src/lib的顶部。

```rust
// in lib.rs

pub mod first;
```




# Basic Data Layout

链表是什么？

Answer: 

基本上，他是堆上的一堆数据按顺序指向对方。

For Procedural programmer:

Linked lists are something are something procedural programmers shouldn't touch with a 10-foot pole, and what functional programmers use for everything.

链表是过程式程序员不应该碰的东西，而函数式程序员用它来做任何事情。

Fo Functional programmers:

So: It seems fair, we should ask functional programmers for the definition of a linked list. 

So: funcitonal programmer give the definition of List:

**`List a  = Empty | Elam a (List a)`**

Which Reads approximately as " A List is either Empty or an Element followed by a List".

This is a recursive definition expressed as sum type, which is a fancy name for "a type that can have different values which may be different types" 

This is **enum type** in Rust. 

在Rust中的enum， 一个链表要么是空的类型，要么是一个值后面跟着一个链表。
这被称为递归定义类型，表示为和类型。Rust中的enum就是类型系统的和类型。


C语言中的enum的作用是什么？

我们要避免使用泛型来保持简单的实现，只支持存储32位有符号整数。

```rust
// in first.rs

//pub says we want people outside this module to be able to use List
pub enum List {
	Empty,
	Elem(i32, List),
}
```

```
> cargo build

error[E0072]: recursive type `first::List` has infinite size
 --> src/first.rs:4:1
  |
4 | pub enum List {
  | ^^^^^^^^^^^^^ recursive type has infinite size
5 |     Empty,
6 |     Elem(i32, List),
  |               ---- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `first::List` representable

```

可以看到帮助的信息： insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `first::List` representable

## Box

`pub struct Box<T>( _ );`

A pointer type for heap allocation. 


Box用于堆分配，Box为这个分配提供了所有权，当它们超出范围时删除它们的内容。

Creating a recursive data structure:

```rust
#[derive(Debug)]
enum List<T> {
	Cons(T, Box<List<T>>),
	Nil,
}

let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nill))));
println!("{:?}", list); // Cons(1, Box(Cons(2, Box(Nill))))

```

Recursive structures must be boxed, because if the definition of Cons looked like this:

`Cons(T, List<T>),`

递归结构必须装箱，因为如果Cons的定义是这样的 `Cons(T, List<T>)`


这是因为List的大小取决于列表中有多少元素，因此不知道为一个Cons分配多少内存。

通过间接的引入一个Box。它有一个确定的大小，因为知道了Cons需要多大。

```rust
pub enum List {
	Empty,
	Elem(i32, Box<list>),
}

```

但是这是一个非常愚蠢的链表定义，原因有以下几点：


考虑一个包含两个元素的链表：

`[] = Stack`

`() = Heap`

`[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*）`

There are two key issues:

有两个关键问题：

- **We're allocationg node that just says "I'm not actually a Node" (@@@这里或许指的是分配的Empty的地方吧， (Empty, *junk* )@@@@)**


- One of out nodes isn't heap-allocated at  all. (上面的节点A)是在栈上分配的不是在堆上分配的。

表面上看，这两个问题似乎抵消。我们分配了一个额外的节点（Empty *junk*），但是其中一个节点根本不需要分配。


考虑下面可能的布局：

`[ptr] -> (Elem A, ptr) -> (Elem B, *null*)`

在这个布局中，我们现在可以无条件的堆分配节点。与第一个布局的关键区别在于，第二个布局中没有junk(The key difference is the absence of the junk from our first layout.)

What is this Junk? 

Answer:

To understand that , we'll need to look at how an **enum** is laid out in memory.

为了理解这一点，我们需要看看enum在内存中是如何布局的。


一般来说，我们有一个枚举如下：

```rust
enum Foo {
	D1(T1),
	D2(T2),
	...
	Dn(Tn),
}
```


enum Foo需要存储一些整数来索引它表示的枚举的变量`(D1, D2, ..., Dn)`，这里存储的整数就是枚举的标签。enum Foo还需要足够的空间来存储最大的`T1, T2,..Tn`（加上另外的一些额外的空间，以满足对齐要求）。

这里最重要的一点，尽管Empty是一个单独的位信息。（even though Empty is a single bit of information), 为了一个指针和一个元素必须消耗足够的空间， 因为他必须随时准备好称为一个Elem.

因此，第一个布局的堆分配了一个额外的元素，该元素充满了垃圾，比第二个布局消耗了更多的空间。


```
[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*) 

push C

[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*) 
                                            |
                                            |
                                          (Elem C, ptr) -> (Empty, *junk*)
```

所以这里的*junk*就是一个`(Elem(None, Box<List>)) -- (Elem(None, ptr)) , ptr = Box<List>`;

第一个布局中的，第一个节点根本没有被分配，但是，这比总是分配它还要糟糕。

这是因为它给我们提供了一个 non-uniform node 布局。 这对于push && pop node 没有太大的影响，但是对于分割和合并列表有影响。

## 考虑在这两中布局中分割一个列表

```
layout 1:

[Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)

split off C:

[Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)

[Elem C, ptr] -> (Empty *junk*)

layout 2:

[ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)

split off C:

[ptr] -> (Elem A, ptr) -> (Elem B. *null*)

[ptr] ->(Elem C, *null*)
```

Layout 2's split involves just copying B's Pointer to the stack and nulling the old value out.

布局2的拆分只需复制B在栈中存放的指针。并用null将旧的的值替换掉。

Layout 1最终也是做了同样的事情，但是，必须将C从堆内存拷贝到栈内存 合并是链表拆分相反的过程。

链表为数不多的好处之一是，你可以在节点本身中构造元素，然后在不移动它的情况下自由地在列表中移动它。

你只要摆弄指针，element就被被”移动“。布局1破坏了此属性, 我们有理由相信布局1是坏的。

如何重写列表呢？可以这样做

```rust
pub enum List {
	Empty,
	ElemThenEmpty(i32),
	ElemThenNotEmpty(i32, Box<List>),
}
```

这似乎是个更糟糕的注意。最值得注意的是，这使我们的逻辑更复杂，因为现在有一个完全无效的状态:` ElemThenNotEmpty(0, Box<Empty>)` ，还受到我们的元素分配不一致的困惑。

然而他有一个有趣的属性：它完全避免了分配Empty Case, 将堆分配的总数减少了1。

但是不幸的是，这样做会浪费更多的空间。

这是因为前面的布局利用了空指针优化。

我们之前看到，每一个枚举都必须存储一个tag, 来确定内存比特位表示的枚举的变体。但是，如果我们有一种特殊的enum

```rust
enum Foo {
	A,
	B(ContainsANonNullPtr),
}
```

空指针优化开始生效，这样就消除了tag所需的空间。

如果变体是A，则整个枚举都设置为0。否则，是变体B。

这是因为B不可能是0，因为他包含了一个非零指针。(This works because B can never be all 0's, since it contains a non-zero pointer.)

## 你能想到其他能做这种优化的枚举和类型吗？ 

事实上很多！这就是为什么Rust enum 布局完全不明确。还有一些更复杂的enum布局优化, 空指针优化绝对是最重要的！(but the null pointer one is definitely the most important!),意思就是`&, &mut, Box, Rc, Arc, Vec,` 和其他几类重要类型在Rust中没有开销，当将这些放在`Option`中时。

那么，我们如何避免额外的junk呢? 统一分配，并且得到空指针优化？

我们需要更好的区分拥有一个元素和分配另一个列表的概念。

要做到这一点，，我们必须使用更类似`C-like: struct!`

虽然enum 允许我们声明一个可以包含多个类型中的一个类型。但是struct允许我们声明同时包含多个值的类型。

让我们把List分成两种类型： List和Node。


和前面一样，List或者是空或者是一个元素后跟着另一个List。通过用一个完全独立的类型表示
has an element followed by another list, 

```rust
struct Node {
	elem: i32,
	next: List,
}
pub enum List {
	Empty,
	More(Box<Node>),
}
```


理解

`[ptr(list::More(Box<Node<_A_>))] -> (Node(elem _ A _ , List::More(Box<Node<_B_>))) -> (Node(elem _ B _ , List::More(Box<Node<_C_>))) -> (Node(elem _ C _ ,List::More(Box<Node<_D_>))) -> (Node(elem D, List:Empty))`

`List::Empty is 0`，空指针优化。这个也就是链表的尾部大小， 链表的尾部没有分配额外的垃圾，通过enum的空指针优化得到的

`[ptr(list::More(Box<Node<_B_>))]` 链表的头节点的大小也是固定不变的

`(Node(elem _ A _ , List::More(Box<Node<_B_>)))`

`(Node(elem _ B _ , List::More(Box<Node<_C_>)))`

`(Node(elem _ C _ ,List::More(Box<Node<_D_>)))` 有相同的内部布局大小，这里就是所有元素都是均匀分配


让我们检查一下我们的优先顺序：

- 列表的尾巴从不分配额外的垃圾
- enum 是空指针优化的格式
- 所有元素均匀分配

实际上，我们只是构造了我们用来证明第一个布局(The Book)是有问题的布局。

```
> cargo build

warning: private type `first::Node` in public interface (error E0446)
 --> src/first.rs:8:10
  |
8 |     More(Box<Node>),
  |          ^^^^^^^^^
  |
  = note: #[warn(private_in_public)] on by default
  = warning: this was previously accepted by the compiler but
    is being phased out; it will become a hard error in a future release!
```


我们将List标记为public（因为我们希望人们能够使用它），但是不是Node。

可问题是enum的内部是完全公开的，但是我们不允许公开访问私有类型Node。

我们可以将Node的所有内容完全公开，但是通常情况下，我们倾向于将实现细节保密。

让我们使List成为一个结构体，这样我们就可以隐藏实现细节了

```rust
pub struct List{
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
```

因为List是一个只有一个字段的结构，所以他的大小与该字段相同。 Zero-cost abstractions

```
> cargo build

warning: field is never used: `head`
 --> src/first.rs:2:5
  |
2 |     head: Link,
  |     ^^^^^^^^^^
  |
  = note: #[warn(dead_code)] on by default

warning: variant is never constructed: `Empty`
 --> src/first.rs:6:5
  |
6 |     Empty,
  |     ^^^^^

warning: variant is never constructed: `More`
 --> src/first.rs:7:5
  |
7 |     More(Box<Node>),
  |     ^^^^^^^^^^^^^^^

warning: field is never used: `elem`
  --> src/first.rs:11:5
   |
11 |     elem: i32,
   |     ^^^^^^^^^

warning: field is never used: `next`
  --> src/first.rs:12:5
   |
12 |     next: Link,
   |     ^^^^^^^^^^
```

Ok, Complied!  Rust是相当疯狂的，因为据他所知，我们写得所有东西都是完全无用的，因为我们从不使用head, 没有人使用我们的库，因为他是私人的。进一步说，这意味着Link和Node也是无用的。让我们来解决这个问题吧。让我们实现一些代码为List.

# New

为了将实际代码与类型关联起来，使用impl块：

```rust
impl List {
	//TODO, Make code happen
}
```
现在我们只需要弄清楚如何实际编写代码：

```rust

fn foo(arg1: Tyep, arg2: Type2) -> ReturnType{
	// body
}

```

我们首先需要的是一种构造列表的方法。因为我们隐藏了实现细节（struct 中的变量默认都是私有的），所以我们需要将其作为一个函数提供。在Rust中通常的做法是提供一个静态方法，这只是在impl中的一个普通函数：

```rust
impl List {
	pub fn new() -> Self {
			Self { Head: Link::Empty }
	}
}

```

这里有几点注意：

- Self 是impl 后面的类型的别名。这样就可以不用重复自己了，可以看到new方法返回的类型也是Self，还有一个好处就是当类型的名字变更后，这里的函数也不需要变动。 函数体里面用Self也是这个道理

- 我们创建一个结构体实例的方式与声明它的方式大致相同，只是我们不提供它的字段类型，而是用值初始化它们

- We refer to variants of an enum using ::, which is the namespacing operator.

- 函数的最后一个表达式是隐式返回的。这使得简单的函数更加简洁，也可以使用return来提前返回。


# Ownership 

方法是Rust函数的一个特例，因为self参数没有声明类型

```rust
fn foo(self, arg2: Type2) -> ReturnType {
	//body
}
```

对于self有三种不同的形式：`self, &self, &mut self,` 这三种形式代表了Rust中三种主要的所有权形式：

- `self `- value

- `&mut self` - mutable reference

- `&self `shared reference

value represents true ownership. 可以对value做任何你想做的事情。move it, destroy it, mutate it, 或者loan it out via a reference.

当你通过value传递给某物是，他会被移动到新的位置。这个新的位置现在拥有这个值，并且旧的位置不能再去访问这个值。
出于这个原因，大多数的方法都不需要self。 所以当处理完这个list之后不能在去使用这是糟糕的。

可变引用表示你对不拥有的值的临时独占访问权。完全可以对具有可变引用的值执行任何操作，只要操作时他处于有效的状态。这就意味着实际上可以完全覆盖这个值。一个非常有用的特殊情况是将一个值替换为另一个值，我们将大量使用这个值。
唯一对于&mut不能做的是，移动&mut的值。 因为最为常用的对于&mut就是改变值。

&self 共享引用表示对不属于你的值的临时访问。因为你有共享访问的权限，所以不允许对这个值进行任何改变。
&对于不改变的方法来说很有用。

稍后将会看到，**在某些情况下，关于借用规则可以被绕过，这就是为什么共享引用不被称为不可变引用的原因**，实际上可变引用可以称为唯一引用。但是我们发现，将所有权与可变性联系起来，99%的情况下都能获得正确的直觉。

# Push

```rust
impl List{
	pub fn push(&mut self, elem: i32) {
		//TODO 
	}
}

```

创建一个节点来存储我们的元素

```rust
pub fn push(&mut self, elem: i32) {
	let new_node = Node {
		elem: elem,
		next: self.head, // cannot move out of borrowed content 
	};
}

```

这里我们试图把`self.head`移动到新的节点的next中，但是rust不允许我们这么做。
这样会导致self变成为未初始化，当我们结束借用时返回给调用者。这将会出错，这个调用者变成了部分没有初始化。
这也是前面所说的，这是唯一不能在`&mut`中做的事情。

原则上，rust实际上是可以接受这一点的，但是由于Box所有权的关系。
我们需要一些方法来得到head，而不会让Rust发现它不见了。

通过`mem::replace `这个函数非常有用，可以通过另一个值替换一个借用来窃取一个值。

```rust
pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
            //将Link::Empty转移到dest也就是self.head的地方，然后这个函数返回原来的值也就是原来的
            // self.head的值

            // next: self.head, // cannot move out of `self.head` which is behind a mutable reference
            // 这个是因为head中存放的是Box<node>的话，由于Box的所有权的关系，next: self.head将会将self中的head移动给next
            //这是self就会变成为初始化的部分，但是在push函数中我们使用的&mut 这是独占引用不能发生所有权的转移
            // 这里违反了借用规则，所以报错。将self中的值移动走，self中就没有东西了。这是的可变借用就会指向一个没有任何东西的地方。
            //这里需要寻找一种方法避免可变借用检测到这个变化
            // mem::replace
            //Moves src into the referenced dest, returning the previous dest value.
        });
        self.head = Link::More(new_node)
}

```

在这里，通过`replace`将self.head暂时替换为Link::Empty,然后在替换为列表的新的头部。

# Pop

同理pop也需要改变列表， 但是pop需要返回一些东西。但是pop有可能pop一个空的列表时，这是为了处理这种情况，使用Option来处理这种这种情况。

```rust
pub fn pop(&mut self) -> Option<i32> {
	//TODO
}
```

`Option<T>`是一个枚举值，表示可能存在。可以使`Some(T)`, 或者`None.`

我们自己也可以创建自己的枚举类型，同link那样。 因为Option是如此的通用，以至于被隐式导入到每个文件的作用域中，

`Option<T>`中的T是泛型，这意味着可以为任何类型创建一个Option。

所以，如果我们有Link，如何知道他是Empty还是More， 通过模式匹配match

```rust
pub fn pop(&mut self) -> Option<i32> {
	match self.head {
		Link::Empty => {	
			//TODO
		}
		Link::More(node) => {
			//TODO
		}
	};
	unimplemented!()
}
```

pop 必须返回一个值，这里我们没有完成这个函数因此使用`unimplemented!()`, 表示我们还没有完成函数的实现。他是一个宏。
当运行到他时。会使程序`panic!`

无条件的panic是一个发散函数的例子。发散函数不会返回给调用方。因此他们可以用在任何类型都可能存在的地方。
在这里使用`unimplemented!() `取代`Option<T>`的值。

我们不需要在函数中写return，因为函数中的最后一个表达式隐式地表示它的返回值。
也可以像C语言一样显式的返回return
```
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
   |  
```
默认的情况下，模式匹配将尝试将其内容移动到新的分支中，但是我们不能这样做，因为我们在这里不拥有这个值。

Rust的帮助信息说，可以在match的参数上加上&来解决。

```rust
pub fn pop(&mut self) -> Option<i32> {
	match &self.head {
		Link::Empty => {	
			//TODO
		}
		Link::More(ref node) => {
			//TODO
		}
	};
	unimplemented!()
}

```

继续理清这个逻辑。我们需要创建一个Option。

当时Empty是返回`None`。

在More的情况下，需要返回`Some(i32)`,并且改变list的head

```rust
pub fn pop(&mut self) -> Option<i32> {

    match &self.head {
		let result;
		Link::Empty => {	
			result = None;
		}
		Link::More(ref node) => {
			result = Some(node.elem);
			self.head = node.next;
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
   | 
```

我们试图移出node, 但是我们只有一个对它的共享引用。

想想我们到底需要做什么：

 - 检查列表是否为空
 - 如果为空，就返回None
 - 如果不空的话，
    把list的head remove
	remove its elem
    将list's head 替换为他的head的next
    返货Some(elem)

关键的观点是我们想要删除一些东西，这意味着我们我们要通过by value（所有权也就是所有权默认移动的方式）的方式得到list的head。

显然我们不能通过共享引用 去得到,&self.head来操作。 我们只有一个可变引用，所有唯一的方法就是replace.

```rust
pub fn pop(&mut self) -> Option<i32> {
	match std::mem::replace(&mut self.head, Link::Empty)  {
		let result;
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

通常使用分号会使块返回（）

```rust
pub fn pop(&mut self) -> Option<i32> {
	match std::mem::replace(&mut self.head, Link::Empty)  {
		Link::Empty => None,
		Link::More(node) => {
			self.head = node.next;
			Some(node.elem)
		}
	}
}

```

这样就变的更加简洁了

# Test

现在我们需要测试push和pop，Rust和cargo对于测试支持很棒，我们需要做的就是在编写一个函数函数用#[test]注释它。

通常我们会为测试创建一个新的命名空间，以避免和真实的代码冲突。

```rust
mod test{
	#[test]
	fn basic() {
		//TODO
	}
}
```

然后使用cargo test来测试
```rust
mod test{
	#[test]
	fn basic() {
		let mut list = List::new();
		
		//check empty list behaves right
		assert_eq!(list.pop(), None);
		
		// populate list
		list.push(1);
		list.push(2);
		list.push(3);
		
		//chech normal removal
		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), Some(2));
		
		//push some more just to make sure nothing's corrupted
		list.push(4);
		list.push(5);
		
		//check normal removal
		assert_eq!(list.pop(), Some(5));
		assert_eq!(list.pop(), Some(4));
		
		//check exhaustion
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
	}
}

```

```
> cargo test

error[E0433]: failed to resolve: use of undeclared type or module `List`
  --> src/first.rs:43:24
   |
43 |         let mut list = List::new();
   |                        ^^^^ use of undeclared type or module `List`

```
因为我们创建了一个新模块，所以需要显式地pull list来使用。

```rust
mod test{
	use super::List;
	
	#[test]
	fn basic() {
		let mut list = List::new();
		
		//check empty list behaves right
		assert_eq!(list.pop(), None);
		
		// populate list
		list.push(1);
		list.push(2);
		list.push(3);
		
		//chech normal removal
		assert_eq!(list.pop(), Some(3));
		assert_eq!(list.pop(), Some(2));
		
		//push some more just to make sure nothing's corrupted
		list.push(4);
		list.push(5);
		
		//check normal removal
		assert_eq!(list.pop(), Some(5));
		assert_eq!(list.pop(), Some(4));
		
		//check exhaustion
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
	}
}

```
```
> cargo test

warning: unused import: `super::List`
  --> src/first.rs:45:9
   |
45 |     use super::List;
   |         ^^^^^^^^^^^
   |
   = note: #[warn(unused_imports)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running /Users/ABeingessner/dev/lists/target/debug/deps/lists-86544f1d97438f1f

running 1 test
test first::test::basics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
; 0 filtered out
```

警告是怎么回事？显然我们只在测试中使用了List

为了满足编译器，我们应该指示只有在运行时才应该编译整个测试模块

在mode test 的上面加上`#[cfg(test)]`

```rust
#[cfg(test)]
mod test {
    use super::List;
    // everything else the same
}
```

# Drop

Rust使用析构函数来自动清理资源，如果一个类型实现了一个叫做`Drop trait `的trait, 这个类型就有了一个析构函数。
Trait are Rust's fancy term for interfaces.

```rust
pub trait Drop {
	fn drop(&mut self);
}

```

When you go out of scope, I'll give you a scond to clean up your affairs

如果包含了实现Drop的类型，实际上并不需要实现Drop，而只需要调用他们的析构函数。

在这个list的例子中我们需要做的就是drop head of the list.回去试图drop Box<Node>

这些都是自动处理的

但是自动处理会很糟糕。

`list -> A -> B -> C``

当list被删除时，它将尝试删除a, 而a将尝试删除b， 而b将尝试删除c。可以看出这是递归的代码，但是递归的代码会爆栈。

有些人会认为‘这显然是尾递归，任何正确的语言都会确保这样的代码不会破坏stack’ 事实这是不正确的。为了找到原因，我们需要手动实现Drop

```rust
impl Drop for List {
	fn drop(&mut self) {
		// you can't actually explicitly call drop in real Rust code
		// we're pretending to be the compiler!
		self.head.drop(); // tail recursive -- good!
	}
}

impl Drop for Link {
	fn drop(&mut self) {
		match *self {
			Link::Empty => {}, // Done!
			Link::More(ref mut boxed_node) => {
					boxed_node.drop(); //tail recusive -- good!
			}
		}
	}
}

impl Drop for Box<Node> {
	fn drop(&mut self) {
		self.ptr.drop(); // not tail recusive! 删除box中的内容
		deallocate(self.ptr); //删除Box
	}
}


impl Drop for Node {
	fn drop(&mut self) {
		self.next.drop();
	}
}
```

我们不能删除释放Box中内容，当Box被删除后，所以没有办法以尾递归的方式删除！相反，不得不手动编写一个迭代的删除list。
Instead, we're going to have to manually write an iteratibe drop for List that hoists nodes out of theis boxed.

```rust
impl Drop for List {
	fn drop(&mut self) {
		let mut cur_link = mem::replace(&mut self.head, Link::Empty);
		// while let == do this thing until pattern doesn't match
		while let Link::More(mut boxed_node) = cur_link {
				cur_link = mem::replace(&mut  boxed_node.next, Link::Empty);
				// boxed_node goes out of scope and gets droped here;
				// but its Node's next field has been set to Link::Empty
				// so no unbounded recursion occrus.
		}
	}
}
```

# Premature Optimization

我们的drop实际上非常类似于``while let Some( _ ) = self.pop() {}`,这个当然更简单。
但是一旦我们开始泛化我们存储的类型。他们之间有什么不同，以及可能会导致什么性能问题？

pop返回`Option<i32>`，而我们的实现只操纵链接`Box<Node>`。因此，我们的实现只是围绕指向节点的指针移动。
而基于pop的指针将围绕存储在节点中的值移动。如果我们泛化了我们的列表，并且有人会用它来存储非常大的实例，
那么这种操作将非常昂贵。Box能够就地运行drop其内容，所以它不会受到这个问题的影响。由于超大实例正是实际上
使用链表比数组更可取的东西，因此在在这种情况下表现不佳会让人失望。

如果你希望两种实现都具有最好的性能。那么客户已添加一个新的方法`fn pop _ node(&mut self) -> Link`, 从这个方法中可以派生出pop
和drop。

```rust

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
    pub fn new() -> Self{
        Self {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
            //将Link::Empty转移到dest也就是self.head的地方，然后这个函数返回原来的值也就是原来的
            // self.head的值

            // next: self.head, // cannot move out of `self.head` which is behind a mutable reference
            // 这个是因为head中存放的是Box<node>的话，由于Box的所有权的关系，next: self.head将会将self中的head移动给next
            //这是self就会变成为初始化的部分，但是在push函数中我们使用的&mut 这是独占引用不能发生所有权的转移
            // 这里违反了借用规则，所以报错。将self中的值移动走，self中就没有东西了。这是的可变借用就会指向一个没有任何东西的地方。
            //这里需要寻找一种方法避免可变借用检测到这个变化
            // mem::replace
            //Moves src into the referenced dest, returning the previous dest value.
        });
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match 会发生移动语义的转移
        match std::mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) =>{
                self.head = node.next; // 更改Self.head = Link::Empty的值为node.next，达到了删除一个节点的目的

                Some(node.elem) //返回删除的节点中的elem
            }
        }
    }


}

impl  Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link{
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}
#[cfg(test)]
mod test{

    use super::List;

    #[test]
    fn basic(){
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

}
```
