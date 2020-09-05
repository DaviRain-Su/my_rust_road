/// # Rust 的面向对象编程特性
/// 
/// 面向对象编程（OOP, Object-Oriend Programming)是一种程序建模的方法。
/// 
/// 本章的目标：
/// 
/// - 讨论普遍共识的面向对象特性
/// - 学习如何在Rust语言的习惯下实现这些特性
/// - 如何使用Rust来实现面向对象的设计模式 
/// 
/// 面向对象包含的特性：命名对象、封装、继承
/// 
/// ## 对象包含数据和行为
/// 
/// 面向对象编程的定义：面向对象的程序由对象组成。对象包装了数据和操作这些数据的流程。
/// 这些流程通常被称作方法或操作。
/// 
/// Rust中的结构体和枚举包含数据，而impl块则提供了可用于结构体和枚举的方法。
/// 
/// ## 封装实现细节
/// 
/// 面向对象编程思想之一封装，调用对象的外部代码无法直接访问对象内部的实现细节，而唯一
/// 可以与对象进行交互的方法便是通过它公开的接口。使用对象的代码不应当深入对象的内部去改变
/// 数据和行为。封装使得开发者在修改或重构对象的内部实现时无需改变调用这个对象的外部代码。
/// 
/// 在rust中通过pub关键字来决定代码中那些模块、类型
/// 函数和方法是公开的，而默认情况下其他内容都是私有的。
/// 
/// 在不同的代码区域选择是否添加pub关键字可以实现对细节的封装。
/// 
/// 
#[derive(Debug)]
pub struct AvergedCollection { // AvergedCollection被pub标记表明这个是public的，但是
    //内部的成员字段仍然是私有的，
    list : Vec<i32>, // private
    average: f64, // private
}


impl AvergedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
 
    }

    pub fn remove(&mut self) -> Option<i32> {
        // let result = self.list.pop();
        // match result {
        //     Some(value) => {
        //         self.update_average();
        //         Some(value)
        //     },
        //     None => None,
        // }
        self.list.pop().map(|value|{
            self.update_average();
            value
        })
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self){
        let total : i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod test {
    use super::AvergedCollection;
    #[test]
    fn basic(){
        let mut t1 = AvergedCollection::new();
        println!("{:?}", t1);
        for i in 0..10 {
            t1.add(i);
        }
        println!("{:?}, average = {}", t1, t1.average());
        t1.remove();
        println!("{:?}, average = {}", t1, t1.average());
        
    }
}

/// ## 作为类型系统和代码共享机制的继承
/// 
/// 继承（inheritance）机制使得对象可以沿用另一个对象的数据和行为，而无须重复定义代码。
/// 
/// 在Rust中，无法定义一个继承父结构体字段和方法实现的子结构体。
/// 
/// 在Rust中的另一种解决方案是使用triat。
/// 
/// 使用继承的好处有两点：
/// 
/// - 实现代码复用， 可以为某个类型实现某种行为，并接着通过继承来让另一个类型直接复用这一行为。
/// 在rust中使用trait来实现代码的共享。在trait中可以实现方法的定义，而triat中的方法
/// 也可以有默认的方法定义。任何类型实现这个trait可以去重写triat中的方法, 这同子类覆盖父类的方法一样。
/// 
/// -与类型系统有关：希望子类能够被应用在一个需要父类型的地方。**也就是多态：如果一些对象具有某些共同的特性，
/// 那么这些对象就可以在运行时相互替换使用，多态的通用的概念， 它指代所有能够适应多种数据类型的代码。**
/// 
/// 可以在Rust中使用泛型来构建不同类型的对象，并使用trait约束来决定类型必须提供的具体特性，这一计技术
/// 称为**限定参数多态(bounded parametric polymorphism).**
/// 
/// 
/// 传统的继承实现，子类型继承父类型，子类型继承自己根本不需要的一些东西。这是传统的继承的缺点。
/// 
/// 在Rust中，使用triat对象来代替继承。
/// 
/// 
/// ## 使用triat对象来存储不同类型的值
/// 
/// ## 为共有行为定义一个triat
/// 
/// #### 如何使用trait对象
/// 
/// 定义一个triat，这个trait是一些类型的共有的方法抽象出来的。
/// 
/// 定义一个持有trai对象的动态数组，
/// 
/// trait对象能够指向实现了指定trait的类型实例，以及一个用于在运行时查找trait方法的表。
/// 
/// 如何创建trait对象：通过&引用或者Box<T>智能指针等，添加dyn关键字与指定相关trait来创建trait对象。
/// 
/// 为什么需要指针来创建trait对像？（第19章，动态大小类型和size trait中讨论）
/// 
/// 可以这样理解：**triat是一个动态大小的类型**必须使用引用或者智能指针包装为固定大小的类型**
/// 
/// trait对象可以被用在泛型或具体类型所处的位置。无论我们在哪里使用trait对象，Rust类型系统
/// 都会在编译时确保出现在相应位置上的值实现trait对象指定的triat。 因此，无须在编译时知晓所有可能的具体类型。
/// 
/// 在Rust中，对于结构体和枚举而言，他们的字段中的数据与impl块中的行为是分开的；
/// 而在其他语言中，数据和行为往往被组合在名为对象的概念中。
/// 
/// 对于triat对象来说，有些类似于这样的一个数据和行为被组合在一起的一个对象的概念。
/// 因为他在某种程度上组合了数据和行为。但trait对象与传统对象的不同之处在于，我们无法为
/// trait对象添加数据。
/// 
/// 这是因为， trait对象被专门用于抽象某些共有行为，所以它没有其他语言中的对象那么通用。
///  
/// 
/// 
#[test]
fn trait_object_ex0() {
    // 使用triat object来实现多态调用，这样可以在triat object的数据中存放所有实现了
    // 这个triat的类型，trait会在运行时去查找方法调用它的函数。
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
        // 动态数据的元素类型使用新语法Box<dyn Draw>来定义triat对象，
        // 它被用来代表所有被放置在Box中且实现了Draw trait的具体类型。
     
        // 通过在定义动态数组components时指定Box<dyn Draw>元素类型，Screen
        // 实例只会接收那些能够调用draw方法的值。
    }

    impl Screen {
        // run方法中的代码只关心对行为的相应，而不在意值的具体类型。
        // 这一概念与动态类型语言中的“鸭子类型”是十分相似，：如果某个
        // 东西走起来像鸭子， 叫起来也像鸭子， 那么它就是一只鸭子。

        // 通过使用trait对象与类型系统来实现“鸭子类型”有一个明显的优势：我们
        // 永远不需要在运行时检查某个值是否实现了指定的方法，或者担心出现“调用
        // 未定义方法”等运行时错误，Rust根本就不会允许这样的代码通过编译。
        pub fn run(&self){
            // 这个run方法会逐一调用components中每个元素的draw方法。
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}
#[test]
fn trait_object_ex1(){  
    trait Foo{
        fn foo(&self){
            println!("FOO!");
        }
    }

    struct A;
    struct B;
    struct C;
    impl Foo for A {
        fn foo(&self) {
            println!("A-FOO!!");
        }
    }
    impl Foo for B {
        fn foo(&self) {
            println!("B-FOO!!");
        }
    }
    impl Foo for C {
        fn foo(&self) {
            println!("C-FOO!!");
        }
    }
    let a = Box::new(A);
    let b = Box::new(B);
    let c = Box::new(C);

    let mut  vec : Vec<Box<dyn Foo>> = vec![];
    vec.push(a);
    vec.push(b);
    vec.push(c);
    for i in vec.iter_mut() {
        i.foo();
    }

    vec.pop();
    
    for i in vec.iter() {
        i.foo();
    }
}

#[test]
fn trait_yueshu_ex0(){
    // trait object 和 trait 约束的泛型参数之间的区别 
    //
    //也可以使用tirat约束的泛型参数来定义结构体，但是这与triat object来说是不同的。
    //在triat object中。triat object中的数据可以存放任何实现了这个trait的类型。
    //但是对于使用trait约束的泛型参数来定义，泛型参数一次只能被替代为一个具体地类型，
    //而trait对象则允许在运行时填入多种不同的具体类型。
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T: Draw> {
        // 为了使用新定义的Screnn实例，这里被限制在list中存储完全由
        // Buutton类型组成的列表，或完全由TextField类型组成的列表。
        // 如果需要的仅仅是同质集合，那么使用泛型和trait约束是OK的。这段定义会在编译
        // 时被多态化一遍使用具体类型。
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
        where T: Draw
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}

/// # 实现triat
/// 
#[test]
fn impl_trat() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }


    impl Screen {
        pub fn new() -> Self {
            Self { components: vec![] }
        }



        pub fn run(&self){
            for component in self.components.iter() {
                component.draw();
            }
        }

        pub fn add(&mut self, elem: Box<dyn Draw>) {
            self.components.push(elem);
        }
    }

    // Button 
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub lable: String,
    }

    impl Button {
        pub fn new(width: u32, height: u32, lable: String) -> Self {
            Self {
                width, height, lable
            }
        }
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("width = {}, height = {}, lable = {}",
                self.width, self.height, self.lable
            );
        }
    }

    pub struct SelectBox {
        width: u32,
        height: u32,
        option: Vec<String>,
    }
    impl SelectBox {
        pub fn new(width: u32, height: u32, option: Vec<String>) -> Self {
            Self {
                width, height, option
            }
        }
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("width = {}, height = {}, option = {:?}",
                self.width, self.height, self.option
            );
        }
    }

    // 如果不给string实现Draw trait， screnen中就不能去加入 String的实例
    impl Draw for String {
        fn draw(&self) {
            println!("String is  = {}", self);
        }
    }

    let selectbox = SelectBox::new(75, 10, vec![String::from("Yes"), String::from("Maybe"), String::from("NO")]);
    let button = Button::new(50, 10, String::from("OK"));

    // let screen = Screen {
        // components: vec![Box::new(selectbox), Box::new(button)],
    // };

    let mut  screen  = Screen::new();
    
    screen.add(Box::new(selectbox));
    screen.add(Box::new(button));
    
    let button1 = Button::new(30, 30, String::from("123"));
    screen.add(Box::new(button1));
    
    let str  = Box::new(String::from("HI"));
    screen.add(str);
    screen.run();
}

/// # triat 对象会执行动态派发
/// 
/// 对于泛型
/// 
/// Rust编译器会在泛型使用trait约束时执行单态化：编译器会为每一个具体类型生成
/// 对应泛型函数和泛型方法的非泛型实现，并使用这些具体地类型来替代泛型参数。
/// 
/// 通过单态化生成的代码会执行静态派发，
/// 这意味着编译器能够在编译器过程中确定你调用的具体方法。
/// 
/// 对于动态派发
/// 
/// 动态派发下的编译器无法在编译过程中确定你调用的究竟是哪一个方法。
/// 在进行动态派发的场景中，编译器会生成一些额外的代码以便在运行时找出我们希望调用的方法。
/// 
/// 对于动态派发Rust使用triat 对象来实现。
/// 
/// 因为编译器无法知道所有能够用于tirat对象的具体类型，所以它无法再
/// 编译时确定需要调用哪个类型的具体方法。rust会在运行时通过tirat对象内部的指针去定位
/// 具体调用哪个方法。
/// 
/// 动态派发会阻止编译器内联代码，进而使得部分优化操作无法进行。
/// 
/// ## trait对象必须保证对象安全
/// 
/// 
/// tirat对象的使用需要注意的是，
/// 
/// trait对象， **只能把满足对象安全的trait转换为trait对象。**
/// 
/// 如果一个trait中定义的所有方法满足下面两条规则，那么这个triat就是对象安全的：
/// - 方法的返回类型不是Self。
/// - 方法中不包含任何泛型参数
/// 
/// 
/// 解释第一条：
/// 
/// 关键字Self是一个别名，它指向了实现当前trait或方法的具体类型。
/// triat对象必须是对象安全的，因为Rust无法再我们使用trit对象时确定实现这个triat的具体
/// 类型究竟是什么。由于trait对象忘记 Self的具体类型，所以编译器无法再trait方法返回Self
/// 时使用原来的具体类型。
/// 
/// 解释第二条：
/// 
/// 对于trait方法中的泛型参数而言，我们会在使用时将具体类型填入泛型所处的位置，这些具体
/// 类型会被视作当前类型的一部分。由于trait对象忘记了类型信息，所以我们无法确定被填入泛型参数处
/// 的类型究竟是哪一个。
/// 
/// 
/// 标准库中的Clone trait就是一个不符合对象安全的例子。
/// ```
/// pub trait Clone {
///     fn clone(&self) -> Self;
/// }
/// ```
/// 
/// 编译器会在你使用trait对象时，指出违反了对象安全规则的地方。
/// 
pub fn triat_object_safe() {
    pub struct Screen {
        // pub components : Vec<Box<dyn Clone>>,
        //the trait `std::clone::Clone` cannot be made into an object
        // the trait cannot be made into an object because it requires `Self: Sized`
    }
    // 错误信息表明我们不能按照这种方式将Clone trait 用作trait对象。
}

/// # 实现一种面向对象的设计模式
/// 
/// 
/// - 状态模式的实现：
/// 
/// 状态模式是一种面向对象的设计模式，
/// 关键点是，
/// 一个值拥有的内部状态由数个状态对象表达而成，而值的行为随着内部状态的改变而改变。
/// 这种设计模式会通过状态对象累共享功能：在Rust中使用了结构体与trait而不是对象与集成类实现这一特性。
/// 
/// 每个状态对象都会负责自己的行为并掌握自己转换为其他状态的时机。而持有状态对象的值则对
/// 状态的不同行为和状态转换的时机一无所知。
/// 
/// 使用状态模式意味着在业务需求发生变化时，我们不需要修改持有状态对象的值，或者使用这个值的代码。
/// 我们只需要更新对象的代码或增加一些新的状态对象，就可以改变程序的运转规则。
/// 
/// 
/// # Demo， 采用增量模式的开发过程来实现一个用于发布博客的工作流。
/// 
/// 1. 在新建博客文章时生成一份空白的草稿文档。（草稿状态）
/// 
/// 2. 在草稿撰写完毕后，请求对这篇草稿状态的文章进行审批。（草稿状态 --> 审批状态）
/// 
/// 3. 在文章通过审批后正式多对外发布。 （审批状态 --> 发布状态）
/// 
/// 4. 仅返回并打印成功发布后的文章，而不能意外地发布没有通过审批的文章。(规定)
/// 
/// 除了上面描述的流程，任何其他对文章的修改行为都应当时无效的。
/// 例如， 假设有人试图跳过审批过程来直接发布草稿状态的文章，那么我们的程序应当
/// 阻止这一行为并保存文章的草稿状态。
/// 
/// 
/// 用户在使用这个Blog库的时候，涉及到数据类型只有Post类型，这个类型会采用状态模式，
/// 它持有的值会是三种不同的文章的状态对象中的一个：草稿、等待审批或发布。Post类型
/// 会在内部管理状态与状态之间的变化过程。虽然状态变化的行为会在用户调用Post实例的对应
/// 方法是发生，但用户并不需要直接对这一过程进行管理。这同样意味着用户不会因为状态
/// 而出错，比如在审批未完成前发布文章。
/// 
/// ## 定义Post并新建一个处于草稿状态下的新实例
/// 
/// 定义一个存储内容的公共结构体Post
/// 
/// ## 存储文章的内容
/// 
/// add_text()
/// 
/// ## 确保草稿的可读内容为空
///  
/// 
/// 即便用户调用add_text方法为文章添加了一些内容，但只要文章处于草稿状态。
/// 我们就需要在用户调用conext方法时范湖一个空的字符串
/// 
/// ## 请求审批文章并改变其状态
/// 
/// 请求审批文章的功能会将文章的状态从Draft编为PendingReview
/// 
/// ## 添加approve 方法来改变content的行为
/// 
/// approve方法与request_review方法类似，他会执行状态的审批流程，并将state设置
/// 为当前状态审批后返回的值。
/// 
/// ## 状态模式的权衡取舍
/// 
/// 基于状态模式，我们可以免于在Post的方法调用或使用Post的代码中添加match表达式。
/// 当业务需要新增状态时，我们也只需要创建一个新的结构体并为他实现trait的各种方法即可。
/// 
/// 状态模式的缺点：
/// 
/// 1. 因为状态实现爱内了状态间的转移，所以某些状态之间是相互耦合的。如果我们希望在PendingReview和
/// published 之间添加一个Scheduled状态，那么我们就需要修改PendingReview中的代码来转移到
/// Scheduled状态。
/// 
/// 2. 我们需要重复的实现一些代码逻辑。你也会会试着提供模式实现，让State trait和request_review和approve 方法
/// 默认返回self，但这样的代码违背了对象安全的原则，因为triat无法确定self的具体类型
/// 究竟是什么。如果我们希望将State当作trait对象来使用，那么它的方法就必须全部是对象安全的。
///  
///  
pub struct Post {
    state: Option<Box<dyn State>>, //state用于存放状态，这里需要去做状态的转换
    // 不使用Pub, 是要控制用户访问content的权限，控制用户访问字段的数据时的具体行为
    content: String, // 存放文章的内容
}
impl Post {
    pub fn new() -> Self {
        Self {
            // 这里将状态设置为了Draft，任何创建出来的Post实例
            // 都会从草稿状态开始。
            // 因为Post的state字段是私有的
            // , 所以用户无法采用其他状态来创建Post。
            state: Some(Box::new( Draft {} )), // 私有的
            content: String::new(),           
        }
    }

    //该方法不依赖与文章当前所处的状态，所以他不是状态模式的一部分。
    // pub fn add_text(&mut self, text: &str) {
    //     self.content.push_str(text);
    // }
    pub fn add_text(&mut self, text: &str) {
        // self.state.take().unwrap().add_text(&mut self, text);
        if let Some(s) = self.state.take(){
            self.state  = Some(s.add_text(self, text)); // ?? 为什么是self, 而不是&mut self 
        }
    }

    pub fn request_review(&mut self) {
        // 做状态的转换
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.approve());
        }
    }

    // 更新Post的content方法，在该方法中委托调用State的content方法
    // 
    // 因为我们希望所有的规则在State相关结构体的内部实现，所以我们会调用
    // state值的content方法，并将Post实例本身作为参数传入，最后将这个方法返回的值作为结果
    // 
    // 这里代用了Option的as_ref方法，因为我们需要的只是Option中的引用，而不是他的所有权。
    // 由于state的乐行是Option<Box<dyn State>>, 所以我们会在调用as_ref时得到Option<&Box<State>>
    // 如果这段代码中没有调用as_ref， 那么就会导致编译时的错误，因为我们不能将state从函数参数
    // 的借用&self中移出
    //
    // 接着调用了unwarp方法，这是因为Post的具体示例中保证了方法调用结束时state总是会有一个
    // 有效的Some值，所以可以 确定的调用unwrap不会发生panic. 
    //
    //
    // 又调用了&Box<dyn State>的content方法，由于解引用会一次作用于&, Box, 所以
    // 我们最终调用的content方法来自实现了 State trait的具体类型。
    // 这意味着我们需要在 State trait 的定义中添加content方法，并在这个方法的实现中
    // 基于当前状态决定究竟返回那些内容。

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take(){
            self.state = Some(s.reject());
        }
    }
}

/// 接口，不同的状态需要去实现这个State trait
/// State trait 定义了所有文章状态共享的行为，
/// Draft,
/// PendingReview,
/// Published
/// 都会实现 State trait
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;    
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // 这个方法需要加上生命周期，因为返回的是Post结构中的content的&str，
    // 这个方法的实现爱内需要接受post的引用作为参数，并返回post中某一部分的引用
    // 作为结果，因此该方法中返回值的生命周期应该与post参数的生命周期相关。
    fn content<'a>(&self, _post: &'a Post) -> &'a str{
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn add_text(self : Box<Self>, _post: &mut Post, _text: &str) -> Box<dyn State>;
}

/// 草稿状态
struct Draft {}

/// 为草稿状态实现这个 State，这个tarit是状态转换的
impl State for Draft {
    // 消耗传入的草稿状态，变为审校状态
    // self : Box<Self> 作为第一个参数，这个语法意味着该方法只能被包裹着当前类型的
    // Box实例调用，他会在调用过程中获取Box<Self>的所有权并使旧的状态失效，从而将Post
    // 的状态值转换为一个新的状态。
    //
    // 为了消耗旧的状态，, request_review方法需要获取状态值的所有权，这也正是Post的state
    // 字段引入Option的原因： Rust不允许结构体中出现未被填充的值。
    // 
    // 可以通过Option<T>的take方法来取出state字段的Some值，并在原来的位置留下一个None。
    // 这样做使我们能够将state的值从Post中移出来， 而不单单只是借用它.
    //
    // 这样做也确保了Post无法再我们完成状态转换后再次使用旧的state值. 
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 这里Draft也实现了content只不过这里是默认的实现方式

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // fn add_text<'a>(&self, post: &'a mut Post, text: &str) -> Box<dyn State>{
        // post.content.push_str(text)
    // }
    fn add_text(self : Box<Self>, _post: &mut Post, _text: &str) -> Box<dyn State> {
        _post.content.push_str(_text);
        self
    }
}

struct PendingReview{}

impl State for PendingReview {
    // 对于一篇已经处在PendingRevirw状态下的文章，发起审批请求并不会改变该文章的当前状态

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // PendingReview实例会在调用approve时返回一个包裹在Box
    // 内的Published结构体的新实例
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }

    // 这里PendingReview也实现了content只不过这里是默认的实现方式
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft{})
    }

    fn add_text(self : Box<Self>, _post: & mut Post, _text: &str) -> Box<dyn State> {
        self
    }
}


struct Published {}

impl State for Published {
    // 返回的都是本身，因为处于Publish的状态下的文章不应当本这些操作改变状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // 同上注释
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text(self : Box<Self>, _post: & mut Post, _text: &str) -> Box<dyn State> {
        self
    }
}
#[test]
fn exmaple(){
    
    // 主的流程逻辑
    let mut post = Post::new();

    // 此时出于草稿状态， 可以添加文字，到那时不能获取到文章的内容
    post.add_text("I ate a salad for lunch today");
    post.add_text("1234567890");
    assert_eq!("", post.content());

    // 此时是一个状态的转换，从草稿状态转换为审批状态， 此时已然无法获得内容
    post.request_review();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // post.reject();
    // assert_eq!("", post.content());

    // 此时是一个状态的转换，从审批状态到发布状态，此时可以获得文章的内容
    post.approve();
    // post.approve();
    assert_eq!("I ate a salad for lunch today1234567890", post.content());
    // assert_eq!("", post.content());
    //-----------implement blog post -------------------------------

}


/// # 将状态和行为编码成类型
/// 
/// 这里将状态编码为类型，而不是完全封装状态与转移过程以使外部对其一无所知。
/// 结果，Rust的类型检查系统将会通过编译时错误来阻止用户使用无效的状态，比如在需要
/// 使用已发布文章的场合误用处于草稿状态的文章。
/// 
/// 
/// 我们仍然希望通过Post::new 来创建出状态为草稿的新文章，并保留向文章
/// 中添加内容的能力。但我们不是让草稿的content方法返回一个空字符串。而是根本
/// 不会为草稿提供content方法。基于这样的设计，用户会在试图读取草稿内容时得到的方法不存在的编译
/// 错误。这使得我们不可能在产品中意外暴露出草稿内容，因为这样的代码连编译都无法通过。
/// 
/// 
/// ## 将状态转移实现为不同类型之间的转换
///  
/// 
/// 仍然希望一篇草稿状态的文章能够在得到审批后发布，而一篇处于待审批状态的文章
/// 则不应该对外显示任何内容。
/// 
/// 由于request_review和approve方法获取了self的所有权，所以他会消耗DraftPost和
/// PendingReviewPost实例，并分别将自己转换为PendingReviewPost和已发布的Post。
/// 
/// 
/// 
/// # 总结
/// 
/// 使用trait对象来实现部分面向对象的特性。
/// 
/// 动态派发通过牺牲些许的运行时能赋予代码更多的灵活性。
/// 可以利用这种灵活性来实现有助于改善代码可维护性的面向对象模式
/// 
/// 由于Rust具有所有权等其他面向对象语言没有的特性，所以面向对象的模式
/// 仅仅是一种可用的选项。而并不总是最佳的实现方式。
/// 
/// 
pub mod test1 { 
    #[test]
    fn basic(){
        let mut post = Post::new();
        
        post.add_text("I ate a salad for lunch today");
        // assert_eq!("", post.content()); // error , DraftPost no content method 
        
        let post = post.request_review();
        
        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }

    /// 这里将状态编码为了结构体类型，所以这两个结构体不再拥有之前的state字段
    /// 新的Post 结构体将会代表一篇已发布的文章，它的content方法被用来返回内部content字段的值
    /// 
    pub struct Post {
        content : String,
    }
    pub struct DraftPost {
        content : String,
    }
    pub struct PendingReviewPost {
        content: String,
    }

    impl Post {
        /// 定义了关联函数new， 但是返回的是一个DratPost实例，而不再是Post实例
        /// 
        /// 没有任何直接返回Post的函数，所以我们暂时无法创建出Post实例。
        ///  
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }
    /// 由于DraftPsot的content字段是私有的。
    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }
        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}