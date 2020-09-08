/// # chapter7 结构化编程
/// 
/// ## 面向对象风格编程
/// 
/// 
/// rust支持面向对象风格的编程， 传统面向对象中父子继承是为了实现代码复用和多态，
/// 其本质在类型系统概念中属于子类型多态，而Rust使用trait和泛型提供参数化多态就
/// 就完全满足这个需求。
/// 
/// 对于代码复用， Rust提供泛型单态化和triat对象来避免代码重复，从而支持代码复用。
/// Rust还提供了功能强大的宏系统帮助复用代码，甚至还可以使用一些设计模式来避免
/// 代码重复。
/// Rust还实现了一种名为**特化**，的功能来增强代码的高效复用。
/// 
/// 总结：
/// - 封装， rust提供了结构体，枚举来封装数据， 并可以使用pub关键字定义其字段可见性
/// 提供了impl关键字来实现数据的行为
/// - 多态，通过trait和泛型以及枚举来允许程序操作不同类型的值
/// - 代码复用， 通过泛型，trait对象，宏，语法扩展，代码生成来设计模式
/// 
/// ## 结构体
/// 
/// 结构体和枚举体是Rust中最基本的两种符合类型，这两种复合类型都属于同一种概念，
/// 他们都属代数数据类型(ADT, Algebraic Dat Type). 
/// 
/// ### 代数数据类型之积类型
/// 
/// 代数数据类型就是值具备了代数能力的数据类型，即数据类型可以进行代数运算并马努一定运算
/// 规则，正是这一点保证了数据类型中的许多性质是可以**复用**的。比如一个结构体中包含的成员
/// 都是拥有复制语义的简单原始数据类型，那么这个结构体也是可以通过派生属性#[derive]来放心
/// 的为其实现Copy
/// 
/// Rust 中的结构体属于代数数据类型中的积类型， 积类型是来自范畴论的术语
/// 积类型， 可以通过乘法原理来理解
/// 描述的是，做一件事需要分成很多步，每一步都相互依赖，他表示的是一种组合。
/// 积类型代表的是一种数据结构的复合方式
/// 
/// ## 使用结构体进行面向对象风格编程
/// 
/// 实现在终端输出指定颜色的字符
/// 
/// ```
/// let hi = "hello".red().on_yellow();// 表示的是黄色背景下的红色字符串hello.
/// println!("{}", hi);
/// ``` 
/// 
/// 在终端显式带颜色的字符，使用的ANSI转移序列， ANSI转移序列就是指如ESC和[组合而成的字符
/// 序列，可以实现在屏幕上定位光标或改变输出字符颜色等功能，所以也被称为控制字符。被定义与
/// ASCII码中。
/// ESC有三种表示方式： 
/// - 在shell中表示为\e.
/// - 在ASCII十六进制表示\x1B
/// - 在ASCII八进制表示为\033
/// 
/// 
/// 在终端显式带指定颜色的字符hello， 
/// 
/// echo "\e[31;43mHello\e[0m"
/// echo "\x1B[31;43mHello\x1B[0m"
/// echo "\033[31;43mHello\033[0m"
/// 
/// \x1B为前缀，表示这是一个ANSI控制序列的开始， 用分号相隔的31;43属于颜色代码， 31是
/// 前景色，代表红色， 43位背景色， 代表黄色，字面m为结束符，最后的\x1B[0m结尾代表重置
/// 全部属性，表示一个ANSCi控制序列的结束。
/// 
/// 
/// 实现步骤；
/// 1 定义一个结构体，来封装动态变化的两部分数据
/// 2 为此结构体定义指定颜色的方法，如， red， on_yellow
/// 3 为了实现直接在字符串字面量上链式调用red和on_yellow方法，就必须
/// 为&'a str类型也实现red和on_yellow方法
/// 4 为结构体实现方法， 用于组装ANSI字符串序列
/// 5 打印结果
/// 
///  
/// 
///  
#[test]
pub fn example() {
    // 积类型 Struct
    #[derive(Debug, Clone,Copy)]
    struct Book<'a> { // 所有的字段都是复制语义的
        name: &'a str,
        isbn: i32,
        version: i32,
    }

    let book  = Book {
        name: "the Rust book", isbn: 20181212, version: 1
    };

    let book2 = Book {
        version: 2, ..book // 结构体跟新语法.., book的所有权并没有被转移， 其中
        // 所有的字段都是具有复制的语义的， 这里不会发生所有权转移， 
        // 结构体跟新语法允许使用".."语法来减少代码重复。
    };

    println!("{:?}", book);
    println!("{:?}", book2);

    // 如果结构体中使用了移动语义的成员字段，则不允许实现Copy
    #[derive(Debug, Clone)]
    struct BookNoCopy { //Rust不允许包含String类型的字段实现Copy,
        name: String,
        isbn: i32,
        version: i32,
    }

    let book = BookNoCopy {
        name: "the rust book".to_string(), 
        isbn: 20181212,
        version: 1,
    };

    let book2 = BookNoCopy {
        // version: 2,
        name : String::from("hello, world"),
        ..book 
        // ..book // there will be move , this book name have been move book2
    };

    println!("{:?}", book2);
    // println!("{:?}", book);// error, book, name have move to book2
}



pub mod color;
pub mod enum_color;

/// # 枚举体
/// 
/// ## 代数数据类型之和类型
/// 
/// 枚举体属于代数数据类型中的和类型。
///  
/// 和类型可以借助加法原理来理解
/// 
/// 积类型表示逻辑与（合取），和类型就表示逻辑或（析取）。
/// 
/// Rust中用来消除空指针的Option<T>类型就是一种典型的枚举体
/// Option<T>，代表有和无之和，将两种不同的类型构造为一种新的符合类型。 
/// 
/// 和结构体不同的是，枚举体中的成员是值，而非类型，一般把他们叫做**变体**, 
/// 使用枚举可以更方便地实现多态。
/// 
/// 重构之前的Colr
/// 
/// - 使用枚举体来管理颜色，而不是直接在具体地方法中使用颜色代码
/// - 使用模式匹配代替if来确认结构体中的fgcolor和bgcolor的设置情况
/// - 可以支持字符串设置颜色
/// 
/// ##  析构顺序
/// 
/// 结构体中的字段是如何析构的？
/// 
/// Rust中变量的析构顺序是和其声明顺序相反的，但并非所有的类型都按这个熟悉顺序来析构。
/// Q7: Newtype模式的优点
/// A7：
/// 1. 隐藏实际类型，限制功能。使用Newtype模式包装的类型并不能被外界访问，，除非提供相应的方法.
/// 2. 明确语义。比如可以将f64类型包装为Miles(f64)和Kilometers(f64), 分别代表英里和千米，这样的语义提升是零成本的，没有多余的性能开销。
/// 3. 使复制语义的类型具有移动语义， 比如本来f64是复制语义，而包装为Miles(f64)之后，因为
/// 结构体本身不能被自动实现为Copy， 所以Miles(f64)就成了移动语义。
/// 
/// ### 本地变量
/// 
/// 本地变量遵循先声明后析构的规则，实际上这也是因为栈结构先进后出的特性。
/// ```
/// struct PrintDrop(&' static str)
/// impl Drop for PrintDrop {
///     fn drop(&mut self) {
///         println!("Dropping {}", self.0);    
///     }
/// }
/// 
/// let x = PrintDrop("x");
/// let y = PrintDrop("y");
/// ```
pub mod test {
    
    #[test]
    fn drop_test(){
        struct PrintDrop(&'static str);
        impl  Drop for PrintDrop {
            fn drop(&mut self) {
                println!("Dropping {}", self.0);
            }
        }
        
        // let x = PrintDrop("x");
        // let y = PrintDrop("y");
        // output 
        // Dropping y
        // Dropping x
        
        // 元组

        // let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
        // let tup2 = (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));
        // output
        // Dropping x
        // Dropping y
        // Dropping z
        // Dropping a
        // Dropping b
        // Dropping c
        //
        // 元组整体的析构顺序和局部变量的析构顺序一致的，但是元组内部元素的析构顺序
        // 则和局部变量的析构顺序相反，元组内部是按元素的出现顺序依次进行析构的。
        // let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
        // let tup2 = (PrintDrop("x"), PrintDrop("y"), panic!());
        // output 
        // Dropping x
        // Dropping a
        // Dropping b
        // Dropping c
        //
        // panic 会引起线程崩溃， 线程崩溃，触发了tup2的提前析构，此时tup2不算是一个
        // 完整的元组，这种提前析构的顺序正好和局部变量的析构顺序一致：先声明的元素后析构
        // 
        
        // 结构体和枚举
        // 
        // 结构体和枚举与元组的析构顺序是一致的
        enum E{
            Foo(PrintDrop, PrintDrop)
        }

        struct Foo {
            x: PrintDrop,
            y: PrintDrop,
            z: PrintDrop,
        }

        // let e = E::Foo(PrintDrop("a"), PrintDrop("b"));
        // let f = Foo {
        //     x: PrintDrop("x"),
        //     y: PrintDrop("y"),
        //     z: PrintDrop("z"),
        // };
        // outout 
        // Dropping x
        // Dropping y
        // Dropping z
        // Dropping a
        // Dropping b

        // 同理， slice类型的集合类型的析构顺序，与元组、结构体、枚举体的析构行为是一致的。

        // 闭包捕获变量

        // 闭包的捕获变量的析构顺序和结构体的析构顺序是一致的。
        // 
        // let z = PrintDrop("z");
        // let x  = PrintDrop("x");
        // let y = PrintDrop("y");

        // let closure = move || {y; z; x; };
        // output 
        // Dropping y
        // Dropping z
        // Dropping x
        
        // 闭包捕获变量的析构顺序和闭包内该变量的排列顺序一致，与捕获变量声明的顺序是没有关系的
        // 这里要和普通函数内局部变量相区分，
        // 闭包和元组、结构体类似，也存在析构顺序变化的情况

        // let z = PrintDrop("z");
        // let x  = PrintDrop("x");
        // let y = PrintDrop("y");

        // let closure = move || {
        //     { let z_ref = &z; }
        //     // 这是因为z在move到闭包之前被借用了，所以需要等待其离开作用域
        //     // 归还所有权之后，才能被move到闭包中。
        //     // 变量被捕获的顺序变为了z -> x -> y, 然后按此顺序再进行析构。
        //     y; x; z;
       
        // }; 
        // output 
        // Dropping z
        // Dropping y
        // Dropping x
    }
}

/// # 常用设计模式 
/// 
/// 设计模式的思想一共涵盖了下面4点：
/// - 针对接口编程
/// - 组合优于继承
/// - 分离变和不变
/// - 委托代替继承
/// 
/// Rust语言本身的设计就非常符合这4点思想，
/// 
/// trait可以强制地实现对接口编程；
/// 泛型和triat限定可以代替继承实现多态；
/// 基于代数数据类型的结构体和枚举在没有继承的情况下一样可以更自由的构造各种类型；
/// 类型系统天生分离了变与不变；
/// 常用的迭代器就是利用委托来代替继承的。
/// 
/// 
/// ### 建造者模式
/// 
/// Rust没有提供构造函数是出于对类型安全的考量。
/// 
/// 在Rust中是可以像函数式语言那样直接绑定值来构造类型实例。
/// 在Rust中而是采用的建造者模式完成复杂类型的构造。
/// 
/// 建造者模式是指使用多个简单对象一步步构建一个复杂对象的模式。
/// 该模式的主要思想就是讲变和不变分离，对于一个复杂的对象，肯定会有不变的的部分，
/// 也有变得部分。将他们分离开，然后依次构建。
/// 
pub mod design_pattern {
    
    // 建造者设计模式,将Circle的构造委托给CircleBuilder构造
    pub struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    pub struct CircleBuilder {
        x: f64, 
        y: f64,
        radius: f64,
    }

    impl Circle {
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
        pub fn new() -> CircleBuilder {
            CircleBuilder {
                x: 0., y: 0., radius: 1.,
            }
        }
    }
    // CircleBuilder实现一些方法，用来修改CircleBuilder的字段中的值，并在修改
    // 完之后返回自身的可变借用，
    // 最后Build方法，他根据CircleBuilder的实例构建最终的Circle实例并返回
    impl CircleBuilder {
        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }
        fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.y = coordinate;
            self
        }

        fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
            self.radius = radius;
            self
        }
        fn builder(&self) -> Circle {
            Circle {
                x: self.x, y: self.y, radius: self.radius,
            }
        }
    }
    #[test]
    fn test_circle(){
        let c = Circle::new()
                        .x(1.0)
                        .y(2.0)
                        .radius(2.0)
                        .builder();
        
        println!("area = {}", c.area());
        println!("c.x = {}, c.y = {}", c.x, c.y);
    }

    #[test]
    fn comand_test() {
        use std::process::Command;

        Command::new("ls")
                .arg("-l")
                .arg("-a")
                .spawn()
                .expect("ls command failed to start");
    }
}

/// # 访问者模式
/// 
/// Rust中另一个重要的模式是访问者模式， Vistor Pattern, 访问者模式用于将数据
/// 结构和作用于结构上的操作解耦。
/// Rust语言自身在解析抽象语法树是就用到了访问者模式。
/// 
mod vistor_pattern {
    // Rust 编译器源码中的访问者模式
    // 这段代码用于构建抽象语法树的，Rust语法中包含了语句(Stmt)， 标识符(Name)，和表达式(Expr)
    // 这些包含在ast模块中类型虽然各不相同，但是他们整体是在描述同一个抽象语法树的结构。
    // 因此，整个抽象语法树就是一个异构的结构，其中的每个语法节点都是不同的类型，对于
    // 这些节点的操作也是各不相同的。语法节点基本是确定的，变化不会太大，但是对节点的操作
    // 需要经常改动。


    // 访问者模式一般包含两个层次：
    // 1. 定义需要操作的元素
    // 2. 定义相关的操作

    mod ast {
        pub enum Stmt {
            Expr(Expr),
            Let(Name, Expr),
        }

        pub struct Name {
            value: String,
        }
        pub enum Expr {
            IntLit(i64),
            Add(Box<Expr>, Box<Expr>),
            Sub(Box<Expr>, Box<Expr>),
        }
    }

    mod visit {
        use super::ast::*;

        pub trait Visitor<T> {
            fn visit_name(&mut self, n: &Name) -> T;
            fn visit_stmt(&mut self, s: &Stmt) -> T;
            fn visit_expr(&mut self, e: &Expr) -> T;
        }
    }

    // ast模块定义了抽象语法树种的全部节点相关的数据结构， 而visit模块中的
    // Visitor triat则定义了相关的操作，所以在解析语法树的时候，只需要为
    // 解析器实现相关的visit方法即可操作相关节点。

    use visit::*;
    use ast::*;
    struct Interpreter;
    impl Visitor<i64> for Interpreter{
        fn visit_name(&mut self, n: &Name) -> i64 {
            panic!()
        }
        fn visit_stmt(&mut self, s: &Stmt) -> i64 {
            match *s {
                Stmt::Expr(ref s) => self.visit_expr(s),
                Stmt::Let(..) => unimplemented!(),
            }
        }
        fn visit_expr(&mut self, e: &Expr) -> i64 {
            match *e {
                Expr::IntLit(n) => n,
                Expr::Add(ref lhs, ref rhs) => 
                    self.visit_expr(lhs) + self.visit_expr(rhs),
                Expr::Sub(ref lhs, ref rhs) => 
                    self.visit_expr(lhs) + self.visit_expr(rhs)
            }
        }
    }
}

/// ## Serde库中的访问者模式
/// 
/// Serde, 是一个对Rust数据节后进行序列化和反序列化的高效框架，
/// Serde的命名就是分别从Serialize（序列化）和Deserialize(反序列化)
/// 两个单词中组合而成的。
/// 
/// Serder之所以称为框架，是因为定义了统一的数据模型，并通过访问者模式开放了
/// 序列化和反序列化的操作接口。Serde支持的数据格式， JSON, XML, BinCOde, 
/// YAML, MessagePack, TOML. 
/// 
/// Serde中序列化和反序列化都是用了访问者模式，
/// Serde中自定义了一些类型来对应可能出现的所有数据类型，包括基本的原生类型，String, option
/// unit， seq, tuple, tuple_strcut, map, struct 等， option代表了Option<T>， 
/// tuple_struct 代表了元组结构体，， seq代表了线性序列，map则的代表了k-v结构的容器
/// 
/// 这些异构的类型构成了Serde框架的统一数据模型。
///  
/// 
/// 通过Deserializer和Visitor两个trait定义了反序列化开放的操作接口，这就涉及Serde框架
/// 利用访问者模式所定义的主要内容，统一的数据模型和开放的操作接口，然后再针对不同的数据格式
/// 实现不同的访问者操作方法。
/// 
/// 
/// 
mod serde_example {
//     trait Deserializer<'a> {
//         type Error;
//     }

    pub trait Deserialize<'de>: Sized {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'de>;
    }
    pub trait Deserializer<'de> : Sized {
        type Error;
        fn deserialize_any<V>(self, visitor: V) 
            -> Result<V::Value, Self::Error> where V: Visitor<'de>;
        fn deserialize_str<V>(self, visitor: V) 
            -> Result<V::Value, Self::Error> where V: Visitor<'de>;
        // ...
    }

    pub trait Visitor<'de> :Sized {
        type Value;
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>;
            // where E : Error,
        // {
            // Err(Error::invalid_type(Unexpected::Bool(v), &self))
        // };
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>;
            // where E : Error,
        // {
            // Err(Error::invalid_type(Unexpected::Str(v), &self))
        // }
        // ... 
    }

    /// JSON格式数据反序列化
    /// 将JSON格式中出现的数据类型统一定义为一个Value枚举体，
    /// 
    /// 反序列化，就是讲JSON格式的字符串解析为Rust数据类型，
    /// 
    /// 访问者模式将数据结构和操作分离开，为代码的扩展提供了极大的便利
    #[derive(Debug, Clone, PartialEq)]
    pub enum Value {
        Null,
        Bool(bool),
        // Number(Number),
        String(String),
        Array(Vec<Value>),
        // Object(Map<String, Value>),
    }

    // impl <'de> Deserialize<'de> for Value {
    //     fn deserialize<D>(deserializer: D) -> Result<Value, D::Error> 
    //         where D : Deserializer<'de>,
    //     {
    //         struct ValueVisitor;
    //         impl <'de> Visitor<'de>  for ValueVisitor{ 
    //             type Value = Value;
    //             fn visit_bool<E> (self, value: bool) -> Result<Value, E> {
    //                 Ok(Value::Bool(value))
    //             }
    //             fn visit_str<E> (self, value: &str) -> Result<Value, E> {
    //                 Ok(Value::String(value.to_string()))
    //             }
    //             // ... 
    //         }
            
    //         deserializer.deserialize_any(ValueVisitor);
    //     }

    //     impl<'de> Deserializer<'de> for Value {
    //         type Error = std::io::Error;
    //         fn deserialize_any<V>(self, visitor: V)
    //             -> Result<V::Value, Error> where V: Visitor<'de> 
    //         {
    //             match self {
    //                 Value::Null => visitor.visit_unit(),
    //                 Value::Bool(v) => visitor.visit_bool(v),
    //                 Value::Number(n) => visitor.visit_any(visitor),
    //                 Value::String(v) => visitor.visit_string(v),
    //                 Value::Array(v) => { visitor.visit_seq(v)},
    //                 Value::Object(v) => { visitor.visit_map(v)},
    //             }
    //         }
    //     }
    // }
}

/// ## RAII模式
/// 
/// Rust的特色是利用Rust进行资源管理
/// 
mod raii_example {
    #[derive(Clone)]
    pub struct Letter {
        text: String,
    }

    pub struct Envelope {
        letter : Option<Letter>, // 表示有信和无信两种状态
    }

    // 表示信件被装车送走
    pub struct PickupLorryHandle {
        done: bool,
    }
    impl Letter {// 写信操作
        pub fn new(text: String) -> Self {
            Self { text }
        }
    }
    impl Envelope {// 装信操作
        pub fn wrap(&mut self, letter: &Letter) {
            self.letter = Some(letter.clone());
        }
    }
    // 表示购买带邮戳的空信封
    pub fn buy_prestamped_envelope() -> Envelope {
        Envelope { letter: None}
    }

    impl PickupLorryHandle {
        // 邮件的装车操作
        pub fn pickup(&mut self, evvelop: &Envelope) {
            // give letter
        }
        // 邮件的寄走操作
        pub fn done(&mut self) {
            self.done = true;
            println!("sent");
        }
    }

    // 表示信封下单装车准备寄走
    pub fn order_pickup() -> PickupLorryHandle {
        PickupLorryHandle {
            done: false, // other handles
        }
    }
    #[test]
    fn test_letter() {
        let letter = Letter::new(String::from("Der RustFest"));
        let mut envelope = buy_prestamped_envelope();
        envelope.wrap(&letter);
        let mut lorry = order_pickup();
        lorry.pickup(&envelope);
        lorry.done();
    }
}

/// 1. letter可能被复制多份并被装到多个信封里，不安全
/// 2. 信封里可能有信，也可能没有信；或者同一信封可能装多封不同的信件，不安全
/// 3. 无法保证一定把信交给邮车，不安全
/// 
/// 
/// 
mod refactor_letter_example {
    pub struct Letter {
        text: String,

    }
    // 空信封
    pub struct EmptyEnvelope{}
    // 已装好信件的信封
    pub struct CloseEnvelope{ letter : Letter }
    pub struct PickupLorryHandle { done: bool }
    impl Letter {
        pub fn new(text: String) -> Self {
            Self { text, }
        }
    }

    impl EmptyEnvelope {
        // 实现将空信封中放入信件， 将letter用于实例化CloseEnvelope，
        // 并转移了letter所有权，确保信件只封装了一次
        pub fn wrap(self, letter: Letter) -> CloseEnvelope {
            CloseEnvelope{ letter }
        }
    }
    // 确保购买的是空信封
    pub fn buy_prestamped_envelope() -> EmptyEnvelope {
        EmptyEnvelope{}
    }

    impl PickupLorryHandle {
        pub fn pickup(&mut self, evenlope: CloseEnvelope) {
            // give letter
        }

        pub fn done(self) {}
    }

    impl Drop for PickupLorryHandle {
        fn drop(&mut self) {
            println!("Sent");
        }
    }

    pub fn order_pickup() -> PickupLorryHandle {
        PickupLorryHandle {
            done: false, // other handles
        }
    }
    #[test]
    fn test_letter() {
        let letter = Letter::new(String::from("Dear RustFest"));
        let envelope = buy_prestamped_envelope();
        let closed_envelope = envelope.wrap(letter);
        let mut lorry =  order_pickup();
        lorry.pickup(closed_envelope);
    }
}