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
 