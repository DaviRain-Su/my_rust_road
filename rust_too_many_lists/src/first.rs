/// # 一个糟糕的单向链表栈
///
///
///
///
/// # 基本布局
///
/// 链表： 它是一大片互相指向的数据，以顺序连接起来。
///
/// 链表是过程式程序员应该用一切代价避免的东西，但是，函数式程序员查询链表的定义是很公平的。
///
/// ## 定义：
///
/// List a = Empty | Elem a (List a)
///
/// 一个列表要么是空的，要么是一个元素接着一个列表。这是用一个复合类型表达的递归定义，
///
/// *复合类型只是"一个可以拥有多种类型的值的类型"另一个叫法*。
///
/// 在Rust中把复合类型称为enum
///
/// 这个enum比C语言中的enum要好很多
///
///
/// 为了保持简洁，会避开泛型。
///
///
/// change before 没有添加Box修饰
///
///
/// pub enum List { // pub使得模块之外的人可以使用这个list
///     // recursive type first::List has infinite size
///     Empty,
///     Elem(i32, List), // recursive without indirection
/// }
///
/// List 是一个非法的递归枚举类型；将内部值包装在一个盒子，使其可表示为合法的递归枚举类型；
/// 把值包装在一个盒子里让它变得可表示，Box
///
///
/// change after 添加Box修饰之后
///
///
/// pub enum List {
///     // recursive type first::List has infinite size
///     Empty,
///     Elem(i32, Box<List>), // recursive without indirection
/// }
///
///
/// # Box
///
/// 用于堆分配的指针类型
///
/// Box<T>, 或被随意称为box 在rust中提供了最简单的堆分配形式，Box提供了当次内存分配的所有权，
/// 当他们超出范围时销毁存放的内容。 Box还确保他们分配的字节不超过isze::MAX
///
/// Exaple:
///
/// 通过创建一个Box将一个值从堆栈移动到堆
///
/// let val : u8 = 5;
/// let boxed: Box<u8> = Box::new(val);
///
///
/// ## 解引用移动
///
/// 如果Box中包装的是一个移动语义的类型，String
/// 在解引用是会发生移动
///
/// 通过解引用将一个从Box移回堆栈
///
///
/// let boxed : Box<u8> = Box::new(5);
/// let val : u8 = *boxed;
///
/// let s = Box::new(String::from("hello, world"));
/// let s2 = *s; //解引用移动
/// // println!("s = {}", s); // error s已经被移动
///
///
///
/// 创建一个递归数据结构
///
///
/// #[derive(Debug)]
/// enum List<T> {
///     Cons(T, Box<List<T>>),
///     Nil,
/// }
/// let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
/// println!("{:?}", list);
///
///
/// 递归结构必须装箱，因为如果Cons的定义是这样的
///
/// Cons(T, List<T>),
///
/// 没用的，这是因为List的大小取决于列表中有多少元素，所以我们无法决定为一个Cons分配多少内存。
/// 通过引入Box<T>, 它有一个确定的大小，我们才知道Cons需要占用多大的内存。
///
///  # 内存布局
///
/// 对于非零大小的值，Box将使用Global分配器进行分配。如果与分配器一起使用的内存布局对于类型是正确的，
/// 那么在Box和由Global分配器分配的原生之间进行两种方式的转换时有效的。
/// 更准确的说，a value : *mut T that has been allocated with Global alloacter with Layout::for_value(&*value)
/// may be convered into a box using Box::<T>::from_raw(value).
/// Conversely, the memory backing a value: *mut T obtained from Box::<T>::into_raw may be deallocated
/// using the Global allocator with Layout::for_value(&*value)
///
/// 只要 T: Sized, Box<T>保证表示为单个指针，并且与C指针（ the C type T*) 也是API兼容的。这意味着
/// 如果你有从 C 调用的 extern "C" Rust函数，你可以使用Box<T> 类型定义这些Rust函数，并使用T* 作为C端对应的类型
/// 作为一个例子，考虑一下这个C header，它声明了创建和销毁某种Foo值的函数
///
///
/// /* C  header */
///
/// // Return ownership to the caller
/// struct Foo* foo_new(void);
///
/// // Takes ownership form the caller; no-op when invoked with NULL
/// void foo_delete(struct Foo*);
///
///
/// 这两个函数可以在Rust中实现如下，这里的， C语言中 struct Foo* 类型被转换为 Box<Foo>
/// 它捕获了所有权约束。还要注意的， foo_delete的可空参数在Rust中表示为 Option<Box<Foo>>，因为
/// Box<Foo>不能为NULL
///
///
/// #[repr(C)]
/// pub struct Foo;
///
/// #[no_mangle]
/// pub extern "C" fn foo_new() -> Box<Foo> {
///     Box::new(Foo)
/// }
///
/// #[no_mangle]
/// pub extern  "C" fn foo_delete(_: Option<Box<Foo>>){}
///
///
///
///
/// 尽管Box<T>具有与C指针相同的表示形式和ABI，但这并不意味着你可以将任意的T* 转换成Box<T>并期望它能够工作。
/// Box<T>值总是完全对齐的，非空指针。 此外，Box<T>的析构函数将尝试使用全局分配器释放值。
/// 一般来说，最佳实践是只对源自全局分配器的指针使用Box<T>
///
/// 注意： 至少在目前，对于在C中定义但是从Rust调用的函数，应避免使用Box<T>类型，在这种情况下，
/// 应该尽可能直接使用与C类型相似的Rust中的类型。
/// 在C定义只是用T*地方使用Box<T>这样的类型可能会导致未定义行为。
///
///
/// 但是实际上是一个非常蠢的列表的定义，
///
/// 考虑一个拥有两个元素的列表：
///
/// [] = 栈
/// () = 堆
/// @ = 垃圾
///
/// [Elem A, ptr] -> (elem B, ptr) -> (Empty *@*)
///
/// 这里有两个关键问题：
///
/// - 我们创造一个"实际上不是个例程"的中断 ？？？？？
/// - 其中的一个元素根本没有分配在堆里， eg: 上面的 Elem A 分配在了栈中
///
///
/// 在表面上，这两个问题近似相互替换，但是我们分配了一个多余的中断，但是还有一个元素完全没有
/// 在堆里的分配
///
/// 考虑到链表的一个潜在的内存布局：
///
/// [ptr] -> (elem A, ptr) -> (elemB,*null*)
///
/// 在这个布局里，我们在堆里分配所有的元素，和第一个布局布局，相比的话，核心的区别是多余的垃圾的消失。
///
/// 那么这个垃圾是什么？
///
/// 为了理解它，我们需要看一看枚举是如何在内存中布局的。
///
/// 通常，我们的枚举都是这个样子的
///
///
/// enum Foo {
///     D1(T1),
///     D2(T2),
///     D3(T3),
///     D4(T4),
///     Dn(Tn),
/// }
///
/// 一个Foo需要保存一个整数，来指出它实际表示的是哪一个变体(D1, D2, D3, ..., Dn). 这个数就是enum的标签（tag）
/// 同时还需要足够大的空间，来存储T1, T2, T3, ..., Tn中的最大元素（以及用来满足内存对齐要求的附加空间）。
///
///
/// pub enum List {
///     // recursive type first::List has infinite size
///     Empty,
///     Elem(i32, Box<List>), // recursive without indirection
/// }
///
/// 这个结构的缺点呢
///
/// 有一个很大的缺陷，尽管只是存储的Empty信息，
/// 但是它却消耗了一个指针和一个元素的内存空间，
///
/// 理解：
///
/// 这里说的是这个节点存储的是List中的Empty代表的链表的结束。
///
/// 但是还有可能在后续继续往链表插入节点，因此这里还需要存储Elem(i32, Box<List>) 这里需要消耗的就是一个指针和
/// 一个元素的内存空间
///
///
/// 因为它要随时准备成为一个Elem. 因此第一种布局在堆里分配了一个充满垃圾的多余元素，比第二种布局消耗更多的空间
///
///
/// 让一个元素不再对中分配，或许也比所有元素都在堆中分配更糟。
/// 这是因为他给了我们一个不一致的节点内存布局，在推入和弹出节点时这并没有问题，但是在分割和合并列表时确实会有影响
///
///
/// 考虑在两种布局分别分割一个列表
///
/// ## 布局1
///
/// *Junk* ===> Elem(i32,Box<List>)
///
/// [Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr） -> (Empty *junk*)
///
/// 在C点分割
///
/// [Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
/// [Elem C, ptr] -> (Empty *junk*)
///
/// ## 布局2
///
/// [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)
///
/// 在C点分割
///
/// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
/// [ptr] -> (Elem C， *null*)
///
/// 布局2的分割仅仅涉及将B的指针拷贝到栈上，并把原值设置为NULL,
/// 布局1做了同样的一件事，但是这里布局1需要把C中在堆中的数据拷贝到栈中，
///
/// 反过来，执行上述操作，就是合并列表
///
/// 列表的优点：
///
/// 可以在节点中构建元素，然后在列表中随意调换它的位置而不需要移动它的内存。
/// 只需要调整指针，元素就被"移动"了，
/// 但是第一个布局毁掉了这个特点。
///
/// 改进的措施1， 可以这样做调整
///
///
/// pub enum List {
///     Empty,
///     ElemThenEmpty(i32),
///     ElemThenNotEmpty(i32, Box<List>),
/// }
///
/// 这么做让逻辑变得更复杂了，具体地说出现了一个完全无效的状态, ElemThenNotEmpty(0, Box(Empty))
/// 还有内存分配模式不一致的问题
///
/// 这个布局的好处是，完全避免了在堆里分配Empty，通过设置ElemThenNotEmpty(0, Box(Empty))
/// 让堆内存分配的数量减少了1，
///
/// **不幸的是，这么做反而浪费了更多的空间，因为之前的布局利用了空指针优化。**
///
/// ## enum的标签
///
/// 每个enum需要一个标签，来指明它代表的是哪一个enum的变体，
///
///
/// enum Foo {
///     A,
///     B(Contains_A_Non_Null_Ptr),
/// }
///
///
/// ## 空指针优化的特点，
/// ??? 到底这里的空指针优化的特点是什么
///
/// 消除标签所占用内存空间。如果变体是A， 那么整个enum就被设置为0，否则，变体是B。
///
/// 这可以工作了，因为B存放了一个非空指针，永远不可能为0，
///
/// 还有很多能进行优化的enum和类型：????
///
/// 但是现在能实现优化的只有空指针优化，
/// 这意味着&, &mut, Box, Rc, Arc, Vec，以及其他一些Rust中的重要类型，都放在一个Option中是没有多余开销的。
///
///
/// 所以要如何避免多余垃圾，统一的分配内存，并且从空指针优化中获益呢？
///
/// 因此这里需要将存储元素和分配列表这个两个想法分开，要做到它，需要向C语言看起， Struct
///
/// enum让我们可以定义一种可以存放多个变体中的一个类型，
/// struct则让我们可以同时存放多种元素的类型，
///
/// 这样吧List分成两个类型： 一个List， 一个是Node
///
/// 和前面一样是，一个List要么是Empty， 要么是跟着一个List， 不过，要通过另一个类型
/// 表示"一个元素跟着一个List"
///
/// 这里我的理解是通过List 相当于重新设计了C语言中指针，要去处理指针为空和不空
///
///
/// struct Node {
///     elem: i32,
///     next: List,
/// }
///
/// pub enum List {
///     Empty,
///     More(Box<Node>), //这里编译不过去，包含了内部类型
/// }
///
///
/// - 列表末尾不分配多余垃圾
/// - enum使用了空指针优化
/// - 所有元素的内存分配已知
///
///
///
/// enum的内部是完全公开的，所以在其中包含内部类型是不允许的，
/// 可以让整个Node都称为公开，但是通常在Rust中，倾向于让实现细节都私有化，
///
///
/// pub struct List {
///     head: Link,
/// }
///
/// enum Link {
///     Empty,
///     More(Box<Node>),
/// }
///
/// struct Node {
///     elem: i32,
///     next: Link,
/// }
///
/// List是一个单值的struct，它的大小和之前的enum List的内存空间占用是完全相同的
///
///
///
///
/// # 创建
///
/// 使用impl语句块把实际代码关联到一个类型上
///
///
///
/// pub struct List {
///     head: Link,
/// }
///
/// enum Link {
///     Empty,
///     More(Box<Node>),
/// }
///
/// struct Node {
///     elem: i32,
///     next: Link,
/// }
///
/// impl List {
///     // TODO :
///
/// }
///
/// // 在Rust中创建对象的通常方法是实现一个impl块中的普通静态函数
///
/// impl List {
///     pub fn new() -> Self {
///         List { head: Link::Empty }
///     }
/// }
///
///
///
/// Self是写在impl右侧的那个类型的别名
/// 创建一个struct的实例的语法和声明struct的语法基本相同，只是我们在每个字段
/// 的后面提供的是值而非他的类型
/// 使用命名空间运算符::来访问enum的变体
/// 函数的最后一个表达式被隐式返回
///
///
/// # 所有权入门
///
///
/// 方法在Rust中是一种特殊的函数，它的第一个参数是self， 并且没有类型声明
///
/// fn foo(self, arg2: T) -> ReturnType {
///     //body
/// }
///
///
///
/// 主要存在三种self可以采用的形式： self， &mut self, &self,
/// 他们代表了Rust中所有权的三种主要形式
/// - self -- 值
/// - &mut slef -- 可变引用
/// - &self -- 共享引用
///
///
/// 一个值代表了真正的所有权，可以对值所你想做的任何事情： 移动它， 销毁它，改变他的内容，或者通过一个引用
/// 借用它。
/// 当通过值传递东西时，他就被移动到了新的位置，这个新位置现在拥有了这个值，并且老位置不能在访问该值。
/// 因此， 对于大部分函数我们都不想使用self -- 如果调用函数让我们无法再访问它， 哪很糟糕。
///
/// 一个可变引用代表了对你不拥有的一个值的临时唯一访问权，当你拥有一个可变引用时，你被允许做几乎任何想做的事，
/// 只要满足该引用的生命周期，被借用者仍然可以看见合法的值。
///
/// 这意味着你可以完全覆盖这个值。
/// std::mem::replace()吧
/// 一个有用的特殊情况是把这个值和另外一个做交换 --- 经常使用这个技巧
/// 唯一不能对&mut做的一件事是不加替换将它的值移出，
/// 对于要对self加以修改的方法，&mut self是很好的
///
/// 一个共享引用代表你对不拥有的值值的临时访问。由于访问时共享的，通常改变任何内容都是不被允许的，
/// 可以吧&想作把值丢到博物馆用于展览，
/// 如果我们只想观察self的值，&是很好的。
///
/// 共享引用称为不可变引用
/// 可变引用称为唯一引用
///
///
/// # Push
///
///
/// /*
/// impl List {
///     pub fn push(&mut self, elem: i32) {
///         //TODO
///     }
/// }
///
///
/// pub fn push (&mut self, elem: i32) {
///     let new_node = Node {
///         elem: elem,
///         next: ???
///     }
/// }
///
///
/// // pub fn push(&mut self, elem: i32) {
/// //    let new_node = Node {
/// //        elem: elem,
/// //        next: self.head, // cannot move out of borrowed content //无法移出租借内容
/// //    };
/// //
/// //    self.head = Link::More(new_node);
/// // }
///
///
///
/// pub fn push(&mut self, elem: i32) {
///   let new_node = Node {
///       elem: elem,
///       next: std::mem::replace(&mut self.head, Link::Empty),
///       //将self.head替换为列表的新头部之前，我们临时将它replace为Link::Empty，
///    };
///
///    self.head = Link::More(new_node);
/// }
///
///
///
/// 尝试把self.head字段移动到next中，但是Rust允许我们这样做，这样会导致我们在租借结束，值返回到其
/// 所有者的时候， self只是部分初始化的。
///
/// 正如前面说的，不能通过&mut做的唯一的一件事；这样做是非常粗野的，rust不允许这样做
///
/// 原则应该是可以，但是Rust阻止这样做。（出于安全性）
///
/// 我们需要某种方法得到head, 而不让Rust发现它已经消失了，
///
/// 这里想Rust黑客Indiana Jones寻求建议
///
/// 通过mem::replace，这个极其有用的函数让我们通过替换将一个值从租借中偷出来
///
///
/// # Pop
///
/// pop要改变列表，还要返回结果。
/// pop还要处理特殊的边界，如果列表时空的时候，为了处理这种情况使用Option
///
///
/// pub fn pop(&mut self) -> Option<i32> {
///
/// }
///
/// Option<T> 是一个表示一个值可能存在的也可能不存在的enum, 它要么是Some(T), 要么是None
///
///
///
/// pub fn pop(&mut self) -> Option<i32> {
///     match self.head {
///         Link::Empty => {
///             // TODO
///         }
///         Link::More(node) => {
///             // TODO
///         }
///     };
///     unimplemented!() // 指出没有完成该函数的实现，unimplemented!是一个宏，会在调用的时候让整个程序panic
///
/// }
///
///
/// 无条件panic是一个发散函数，发散函数永远不会返回到调用者，所以无论一个地方期待何种类型的值，他的返回值都能拿来用
///
/// 不需要在程序中写return, 函数中的最后一个表达式就隐式地成为它的返回值。
///
/// ## 模式匹配默认会移动匹配的值，
///
/// 要避免移动，使用ref node, ref mut node 来引用捕获该值
///
///
/// 为了避免移动使用ref关键字来指明我们想要把node进行引用绑定
///
///
/// pub fn pop(&mut self) -> Option<i32> {
///     match self.head {
///         Link::Empty => {
///             // TODO
///         }
///         Link::More(ref node) => { // 使用关键字ref避免移动
///             // TODO
///         }
///     };
///     unimplemented!() // 指出没有完成该函数的实现，unimplemented!是一个宏，会在调用的时候让整个程序panic
///
/// }
///
///
/// 这里需要创建一个Option，所以要为这个预留一个变量，
/// 在Empty下返回Nonde， 在More情况下，返回Some(i32), 并且改变列表的head
///
///
/// pub fn pop(&mut self) -> Option<i32> {
///     let result;
///     match self.head {
///         Link::Empty => {
///             result = None;
///         }
///         Link::More(ref node) => {
///             result = Some(node.elem);
///             self.head = node.next; // cannot move out of borrowed constant
///                                    // cannot assign to self.head because it is borrowed
///         }
///     };
///     result
/// }
///
///
/// 这里出现了两个不同的错误。？？？？？？
///
/// 1. 都一个在仅仅拥有一个共享引用的情况下就尝试把值移动出node ？？？？
/// 2. 在已经租借了node的引用的时候，还在尝试改变self.head的值 ？？？？
///
/// POP需要的操作
/// - 检查列表是否为空
/// - 如果为空，返回None
/// - 如果非空
///     - 移出list头部
///     - 移出该头部的elem
///     - 将列表的head替换为next
///     - 返回Some(elem)
///
/// 最重要的是，我们要删除东西，这意味着我们需要按值获取list的head。
/// 肯定不能通过由ref node 获取的共享引用做这件事。
///
/// 我们只拥有一个可变引用，所以能移动东西的唯一方法就是替换它。
///
///
///
///
///
/// pub fn pop(&mut self) -> Option<i32> {
///     let reuslt;
///     match std::mem::replace(&mut self.head, Link::Empty) {
///         Link::Empty => {
///             result = None;
///         },
///         Link::More(node) => {
///             result = Some(node.elem); // Copy trait
///             self.head = node.next; // 发生了所有权的转移
///         },
///     };
///     result
/// }
///
///
/// // 这里出错的是什么？？？？？
/// // 在第一次迭代的过程中，我们对Result进行赋值时进行拷贝，因此node没有被改变。
/// // 可以被继续用于下一个操作。现在我们在移动next（它不是Copy), 而在这里在我能碰到elem
/// // 之前消耗掉了整个Box里的值。
///
///
/// // 优化
///
/// pub fn pop(&mut self) -> Option<i32> {
///     match std::mem::replace(&mut self.head, Link::Empty) {
///         Link::Empty => None,
///         Link::More(node) => {
///             self.head = node.next; // 通过Box管理具有了移动语义，发生了所有权的转移
///             Some(node.elem) // error: use of moved value: node
///         },
///     }
/// }
///
///
///
/// 在所有权中，当移动值的时候，就无法再使用它，对于某些类型来说，这是完全合理的。
///
/// 对于整数等基本类型来说，没有所有权语义，被标记为Copy trait。
/// Copy类型可以通过按位复制进行完整的拷贝。
/// 因此，当被移动的时候，老的值仍然是可用的。
/// 结果就是，可以将一个Copy类型从引用移出老的值依旧可用。
///
///
/// 正确的操作方法是将整个节点从Box中取出来， 这样就可以安全的将它拆开了
///
///
/// pub fn pop(&mut self) -> Option<i32> {
///     match std::mem::replace(&mut self.head, Link::Empty) {
///         Link::Empty => None,
///         Link::More(boxed_node) => {
///             let node = *boxed_node; //Box中存放的是Copy类型的值不会发生解引用移动
///             self.head = node.next;
///             Some(node.elem)
///         },
///     }
/// }
///
/// Box在Rust里很特殊，因为他是固定语言里的一部分，编译器可以让对它做一些其他任何类型都不能做的事
/// 我们实际上已知在做这件事： DerefMove,
/// 当你拥有一个指针类型时，可以通过*或.来获得它的内容。
/// 通常可以获得一个Deref或者DerefMut,分别对应共享和可变引用。
///
/// 但是Box完全拥有它的内容，可以通过解引用将内容移出。
///
/// # 测试
///
///
///
/// # Drop
///
/// Drop trait 析构的特性
///
/// pub trait Drop {
///     fn drop(&mut self);
/// }
///
/// 当对象退出作用域的时候，会自动清理
///
/// list -> A -> B -> C
///
/// 当列表被丢弃的时，会先丢弃A，然后B，最后C
///
///
/// impl Drop for List {
///     fn drop(&mut self) {
///         list.head.drop();//在实际的代码中不能显示的调用drop
///         //尾递归
///     }
/// }
///
/// impl Drop for Link{
///     fn drop(&mut self) {
///         match list.head {
///             Link::Empty => {},
///             Link::More(ref mut boxed_node) => {
///                 boxed_node.drop();// 尾递归
///             }
///         }
///     }
/// }
///
/// impl Drop for Box<Node> {
///     fn drop(&mut self) {
///         self.ptr.drop();// error 不是尾递归
///         deallocate(self.ptr);
///     }
/// }
///
/// impl Drop for Node {
///     fn drop(&mut self) {
///         self.next.drop();
///     }
/// }
///
///
///
/// ????不能在释放内存之后，在丢弃Box的内容，所有没有办法以尾递归的形式进行drop！
/// 因此必须手动编写一个迭代drop， 来把节点从box中拿出来
///
///
/// // impl Drop for List {
/// //    fn drop(&mut self) {
/// //        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
/// //
/// //        while let Link::More(mut boxed_node) = cur_link {
/// //            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
/// //            // boxed_node 在这里退出作用域然后被丢弃
/// //            // 但是其节点的next字段被设置为Link::Empty
/// //            // 所有没有多层递归产生
/// //        }
/// //    }
/// // }
///
///
///
///
///

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
        let new_node = Box::new( Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => { // ??
                // let node = *boxed_node; // ??
                self.head = node.next; // ???
                Some(node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More (mut boxed_node) = cur_link{
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::first::List;
    use super::List;

    #[test]
    fn basics() {

        let mut list = List::new();

        //检查空列表行为正确
        assert_eq!(list.pop(), None);

        // 填充列表
        list.push(1);
        list.push(2);
        list.push(3);


        // 检查通常的移除
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        //push 更多元素来确认没有问题
        list.push(4);
        list.push(5);

        //检查通常的移除
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        //检查完全移除
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}