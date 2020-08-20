// use std::io::_print;

// use std::intrinsics::write_bytes;

/// # Hello World
    ///
    /// println!是一个宏，可以将文本输出到控制台
    ///
    /// ```
    ///  pub fn hello_world() {
    ///         println!("Hello World!");
    ///         println!("I'm a Rustacean!");
    ///     }
    /// ```
pub fn hello_world() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
}

/// # 注释
///
/// Rust中的注释有
/// - 普通注释， 其内容会被编译器忽略掉
///     - //单行注释，注释内容直到行尾
///     - /* 块注释，注释内容一直到结束分隔符 */
/// - 文档注释， 其内容将被解析成Html帮助文档
///     - /// 为接下来的项生成帮助文档
///     - //! 为注释所属于的项(crate, mod, func)生成帮助文档
///
///
pub fn statement() {
    //这是行注释的例子
    //注意有两个斜线在本行的开头
    //在这里面的所有内容都不会被编译器读取

    // println!("hello, world!");

    // 请运行一下， 你看到结果了吗？删除斜线注释，并重新运行

    /*
    这是另外一种注释，块注释，一般而言，行注释是推荐的注释格式，
    不过块注释在临时注释大块代码特别有用，/* 块注释可以嵌套 */，
    所以只需要很少案件就可以注释掉这下main函数中的行。
     */

    //观察块注释是如何简单地堆表达式进行修改的，行注释则不能这样。
    //删除注释分隔符将会改变结果
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100 ? x = {}", x);
}

/// # 文档注释
///
/// 文档注释支持Markdown
/// 文档注释对于大型项目来说非常重要
///
pub fn doc_statement() {

    /// 这里给出一个人类
    ///
    pub struct Person {
        /// 一个人必须由名字，
        name: String,
    }

    impl Person {
        /// 返回具有指定名字的一个人
        ///
        /// # 参数
        ///
        /// * `name` - 字符串切片， 代表人物的名称
        ///
        /// # 示例
        ///
        /// ```
        /// // 在文档注释总，可以书写代码块
        /// //如果向rustdoc 传递 --test参数，他还会帮你测试注释文档中的代码！
        /// use doc::Person;
        /// let person = Person::new("name");
        /// ```
        pub fn new(name: &str) -> Person {
            Person{
                name: name.into(),
            }
        }

        /// 给一个友好的问候！
        pub fn hello(&self) {
            println!("Hello, {}!", self.name);
        }
    }

    let john = Person::new("John");
    john.hello();
}

/// #  格式化输出
///
/// 打印操作由std::fmt里面定义的一系列宏来处理
/// - format!: 将格式化文本写到字符串String
/// - print!: 与format!类似，当将文本输出到控制台(io::stdout)
/// - println!: 与print!类似，但输出结果追加一个换行符
/// - eprint!： 与format!类似，但将文本输出到标准错误(io::stderr).
/// - eprintln!: 与eprint!类似，但输出结果追加一个换行符
///
///格式化的正确性会在编译时检查
///
/// std::fmt包含各种triat，来控制文字显示， 其中两个重要的trait基本形式
///  - fmt::Debug: 使用{:?}标记，格式化文本以供调试使用
///  - fmt::Display: 使用{}标记，以更优雅和友好的风格来格式化文本
///
///
pub fn format_output() {
    // 通常情况下，`{}`会被任意变量内容所替换
    // 变量内容会转换成字符串
    println!("{} days", 32);

    // 用变量替换字符串有多种写法
    // 比如可以使用位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //可以使用命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 可以在 `:` 后面指定特殊的格式
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 可以按指定宽度来右对齐文本
    println!("{number:>width$}", number=1, width=6);

    //println! 会检查使用到的参数数量是否正确
    println!("My name is {0}, {1} {0}", "Bond","James");

    //创建一个包含单个`i32`的结构体
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct `{:?}` won't print ... ", Structure(3));

    // fmt usage
    println!("{}", format!("hello"));
    println!("{}", format!("hello, {}!", "world"));
    println!("{}", format!("The number is {}", 1));
    println!("{}", format!("{:?}", (3, 4)));
    println!("{}", format!("{value}", value = 4));
    println!("{}", format!("{} {}", 1, 2));
    println!("{}", format!("{:04}", 42));
    println!("{}", format!("{1} {} {0} {}", 1, 2));
    println!("{}", format!("{argument}", argument = "test"));
    println!("{}", format!("{name} {}", name = 2));
    println!("{}", format!("{a} {c} {b}", a = 'a', b = 'b', c = 3));

    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5); //这里的带表的是位置参数1处的值
    println!("Hello {1:0$}!", 5, "x"); //同理，这里的5的位置是0， 'x'的位置是1, 0$这是取得是0位置的值
    println!("hello {:width$}!", "x", width = 5);
    println!("{}", format!("hello {:<5}!", "x")); //左对齐
    println!("{}", format!("hello {:*<5}!", "x"));//左对齐
    println!("{}", format!("hello {:*^5}!", "x"));//居中对齐，其余部分补*
    println!("{}", format!("hello {:*>5}!", "x"));//右对齐

    println!("hello {:15}!", format!("{:?}", Some("hi")));

    println!("{}", format!("Hello {:+}!", 5));
    println!("{}", format!("{:#x}", 27));
    println!("{}", format!("Hello {:05}!", 5));
    println!("{}", format!("Hello {:05}!", -5));
    println!("{}", format!("{:#010x}!", 27));

    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("hello {0} is {2:.1$}", "x", 5, 0.01);
    println!("hello {} is {:.*}", "x", 5, 0.01);
    println!("hello {} is {2:.*}", "x", 5, 0.01);
    println!("hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    println!("{}, `{name:.*}` has 3 fractional digits", "Hello", 3, name= 1234.56);
    println!("{}, `{name:.*}` has 2 characters", "hello", 3, name="1234.56");
    println!("{}, `{name:>8.*}` has 3 right-aligned characters", "hello", 3, name="1234.56");

    // let pi = 3.141592;
    println!("Pi is roughly {0:.3}", std::f64::consts::PI);

    let temp = format!("{}", "hello");
    println!("temp = {}", temp);
}

/// # 调试
///
/// 所有的类型，若想用std::fmt的格式，triat打印出来，都要求实现这个trait， 自动的实现只为一些类型提供，
/// 比如std库中的类型，素有其他类型都必须手动实现
///
/// fmt::Debug 这个triat使这项工作变得相当简单，所有类型都能推导(derive, 即自动创建）fmt::Debug的实现。
/// 但是fmt::Display需要手动实现
pub fn debug_print() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} month in a year", 12);
    println!("{1:?} {0:?} in the {actor:?} name",
        "Slater",
        "Christina",
        actor="actor's"
    );

    println!("Now {:?} will pint!", Structure(2));
    println!("Now {:?} will print!", Deep(Structure(3)));

    #[derive(Debug)]
    struct Person<'a>{
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

        fn name(&self) -> &str {
            self.name
        }
        fn age(&self) -> u8 {
            self.age
        }
    }

    let name = String::from("Davirain");
    let age = 25u8;

    let person = Person::new(&name, age);
    println!("Person  = {:?}", person);
    println!("name = {}, age = {}", person.name(), person.age());
}

/// # 显式 Display
///
/// fmt::Display 自定义输出的外观
///
///
pub fn display_example() {
    use std::fmt;

    #[derive(Debug)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct  MinMax(i64, i64);

    impl  fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    #[derive(Debug)]
    struct Point2D{
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    #[derive(Debug)]
    struct Complex{
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let structure = Structure(23);
    println!("Compare structure : ");
    println!("Display: {}", structure);
    println!("Debug: {:?}", structure);


    let minmax = MinMax(0, 14);
    println!("Compare MinMax: ");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} anf the small is {small}",
        small = small_range,
        big = big_range
    );
    //
    // println!("Compare structure: ");
    // println!("Display: {}", big_range);
    // println!("Debug: {:?}", big_range);

    let point = Point2D { x: 3.3, y: 4.4};
    println!("Compare Point2D:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imag: 4.4};
    println!("Compare Complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

/// # 测试list
///
/// 对于一个结构体实现fmt::Display, 其中元素需要一个接一个地处理到，这可能会很麻烦，问题在于每个write!都要生成一个
/// fmt::Result, 正确的实现需要处理所有的Result.Result专门为解决这个问题提供了?操作符
/// write!(f, "{}", value)?;
///
pub fn test_list() {
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0  { write!(f, ", ")?; }
                write!(f, "{}: {}",count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("v = {}", v);

}

/// # 格式化
///
///
pub fn format_example() {
    use std::fmt::{self, Formatter, Display};

    struct  City {
        name: &'static str,
        //维度
        lat: f32,
        //经度
        lon: f32,
    }
    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S'};
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3} {} {:.3} {}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl  Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write!(f, "red:{}, green:{}, blue:{}", self.red, self.green, self.blue)
            write!(f, "RGB ({}, {}, {}) {}", self.red, self.green, self.blue,
                   format!("0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue))
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722},
        City { name: "Oslo", lat: 59.95, lon: 10.75},
        City { name: "Vancouver", lat: 49.25, lon: -123.1},
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90},
        Color { red: 0, green: 3, blue: 254},
        Color { red: 0, green: 0, blue: 0},
    ].iter() {
        println!("{}", *color)
    }
}