/// # 自定义类型
///
/// Rust自定义数据类型主要通过：
/// - struct 定义一个结构体
/// - enum: 定义一个枚举类型
///
/// 常量可以通过const 和 static 关键字来创建
///
///
/// # 结构体
///
/// 结构体有三类，使用struct来创建
/// - 元组结构体，具名元组
/// - 经典C语言风格结构体
/// - 单元结构体，不带任何字段，在泛型中很有用
///
///
pub fn struct_type() {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    impl <'a> Person<'a> {
        fn new(name: &'a str, age: u8) -> Self {
            Person {
                name,
                age,
            }
        }

        fn get_name(&self) -> &str {
            self.name
        }

        fn get_age(&self) -> u8 {
            self.age
        }

        fn print(&self) {
            println!("My name is {}, and age is {}", self.name, self.age);
        }
    }

    //uint struct
    struct Nil;

    #[derive(Debug, Copy, Clone)]
    struct  Point {
        x: f32,
        y: f32,
    }

    impl Point {
        fn new(x: f32, y: f32) -> Self {
            Point{
                x,
                y,
            }
        }

        fn get_x(&self) -> f32{
            self.x
        }

        fn get_y(&self) -> f32 {
            self.y
        }

        fn add_one(&mut self) {
            self.x += 1.0;
            self.y += 1.0;
        }

        fn print(&self) {
            println!("x =  {}, y =  {}", self.x, self.y);
        }


    }
    #[derive(Debug)]
    struct Pair(i32, i32);

    trait  Area {
        type Output;
        fn area(&self) ->Self::Output;
    }

    #[allow(dead_code)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl  Area for Rectangle {
        type Output = f32;
        fn area(&self) -> Self::Output {
            (self.p1.x - self.p2.x).abs() * (self.p1.y - self.p2.y).abs()
        }
    }



    let name = String::from("Davi_rain");
    let age: u8 = 25;
    let person = Person::new(&name, age);
    let my_name = person.get_name();
    let my_age = person.get_age();
    println!("person is {:?}, name is {}, age is {}", person, my_name, my_age);
    person.print();

    let mut point = Point::new(2.3, 3.4);
    println!("point is {:?}", point);
    point.print();
    point.add_one();
    println!("point is {:?}", point);
    point.print();

    let new_point = Point{ x: 1.0, ..point };
    println!("new point is {:?}", new_point);
    println!("new point x is {}, y is {}", new_point.x, new_point.y);
    println!("new point x is {}, y is {}", new_point.get_x(), new_point.get_y());

    let Point { x: x1, y: y1} = new_point;
    let rectangle = Rectangle{
        p1: Point{ x: x1, y: y1},
        p2: point,
    };

    println!("rectangle area is {}", rectangle.area());

    let _nil = Nil;

    let pair = Pair(2, 3);
    println!("The pair is {:?}", pair);

    let Pair(integer, decimal) = pair;
    println!("integer is {}, decimal is {}", integer, decimal);
}


/// # 枚举
///
///
/// enum关键字可以创建枚举类型，该类型的实例只能在数个可能的取值中取一种，
/// 任何一个合法的struct同时也是合法的enum 取值
///
///
pub fn enum_type_example() {
    #![allow(dead_code)]
    enum  WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click{ x: i64, y: i64},
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unload"),
            WebEvent::KeyPress(c) => println!("pressed: {}", c),
            WebEvent::Paste(s) => println!("pasted: {}", s),
            WebEvent::Click {x, y} => {
              println!("clicked at x = {}, y = {}", x, y);
            },
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("hello".to_owned());
    let click = WebEvent::Click {x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // 类型别名，
    // 若使用类型别名，则可以通过其别名引用每个枚举变量
    enum  VeryVerBoseEnumOfThingToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerBoseEnumOfThingToDoWithNumbers;
    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    let _x = Operations::Add;
}

/// #使用use
///
/// 使用use声明，就可以不用写出名称的完整路径
///
///
pub fn use_example() {
    #![allow(dead_code)]
    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have losts of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians works!"),
        Soldier => println!("Soldiers fights!"),
    }
}

/// # C风格用法
///
/// enum也可以像C语言风格的枚举一样使用
///
pub fn the_c_style_enum_example() {
    #![allow(dead_code)]
    enum  Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

/// # 测试实例： 链表
///
///
/// enum 的一个常见用法就是创建链表
///

pub fn example_list_test() {
    use List::*;

    enum  List {
        //Cons: 元组结构体， 包含链表的一个元素和一个指向下一节点的指针
        Cons(u32, Box<List>),
        // Nill, 末节点，表明链表结束
        Nil,
    }

    impl List {
        // 创建一个空的List实例
        fn new() -> List {
            Nil
        }

        // 处理一个List, 在其头部插入新元素，并返回该List
        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            // 匹配一个具体地T类型要好过匹配引用&T
            match *self {
                // 不能得到tail的所有权，因为self是借用的
                //因此使用一个队tail的引用
                Cons(_, ref tail) => 1 + tail.len(),
                //递归的基准情形， 一个长度为0的空列表
                Nil => 0,
            }
        }

        //返回列表的字符串表示
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                  format!("{}, {}", head, tail.stringify())
                },
                Nil => {
                    format!("Nil")
                },
            }
        }
    }


    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length : {}", list.len());
    println!("{}", list.stringify());
}
/// # 常量
///
/// Rust有两种常量， 可以在任意作用域内声明，包括全局作用域，他们都需要显式的类型声明：
///
/// - const， 不可变的值
/// - static， 具有'static生命周期，可以使可变变量
///
/// 有个特例就是"string"字面量，他可以不经改动就被赋给一个static变量，因为他的类型标记：
/// &'static str就包含了所要求的生命周期'static，其他的引用类型都必须特地的声明，使之
/// 拥有'static生命周期，这两种引用类型的差异似乎也无关紧要，因为无论如何，static变量
/// 都要显式的声明
///
///
pub fn static_const_variable() {
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    let n = 16;

    println!("Thi is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n , if is_big(n) { "big"} else { "small"} );

    // 不能修改一个const常量
    // THRESHOLD = 5;
}