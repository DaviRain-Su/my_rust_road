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
///  
mod enum_color{
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Color {
        Red,
        Yellow,
        Blue,
    }

    impl  Color {
        fn to_fg_str(&self) -> &str { // 前景色ANSI码的获得
            match *self { // 这里的*self 并不会转移所有权
                Color::Red => "31",
                Color::Yellow => "33",
                Color::Blue => "34",
            }
        }
        fn to_bg_str(&self) -> &str { // 背景色ANSI码的获得
            match *self {
                Color::Red => "41",
                Color::Yellow => "43",
                Color::Blue => "44",
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ColoredString {
        input: String,
        fgcolor: Option<Color>,
        bgcolor: Option<Color>,
    }

    impl  Default for ColoredString {
        fn default() -> Self {
            ColoredString {
                input: String::default(),
                fgcolor: None,
                bgcolor: None,
            }
        }
    }

    // 为Color实现From, 用于将&str, String类型的字符串转换为Color，
    // 这样做是为了实现通过字符串来设置颜色的需求。
    //
    // 要实现From,需要显式地导入std::convert::From,std::str::FromStr,
    // 和std::string::String, 
    use std::convert::From;
    use std::str::FromStr;
    use std::string::String;

    impl<'a> From<&'a str> for Color {
        fn from(src: &str) -> Self {
            // 这里用到的prase方法，该方法要求目标类型必须实现FromStr, 
            // 所以后面才有了为Color实现FromStr了
            // parase方法会返回Result类型, 如果是Ok<T>类型，则会通过
            // unwrap来获取其中的值，如果是Err<T>类型，则返回指定的默认值Color:Red
            src.parse().unwrap_or(Color::Red)
        }
    }
    impl From<String> for Color {
        fn from(src: String) -> Self {
            src.parse().unwrap_or(Color::Red)
        }
    } 

    // 这是因为parse需要
    impl FromStr for Color {
        type Err = ();
        fn from_str(src: &str) -> Result<Self, Self::Err> {
            let src = src.to_lowercase();
            match src.as_ref() {
                "red" => Ok(Color::Red),
                "yellow" => Ok(Color::Yellow),
                "blue" => Ok(Color::Blue),
                _ => Err(()),
            }
        }
    }

    // 增减了color, on_color方法，使用这两个方法，就可以通过字符串来设置终端文本的颜色
    // color, on_color泛型方法中使用了triat限定<S: Into<Color>>, 这是因为
    // Color实现了From, 所以对于String, &'a str 类型字符串均可通过into方法
    // 转换为Color. 
    trait Colorize {
        fn red(self) -> ColoredString;
        fn yellow(self) -> ColoredString;
        fn blue(self) -> ColoredString;
        fn color<S: Into<Color>>(self, color: S) -> ColoredString;
        fn on_red(self) -> ColoredString;
        fn on_yellow(self) -> ColoredString;
        fn on_blue(self) -> ColoredString;
        fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
    }

    impl Colorize for ColoredString {
        fn red(self) -> ColoredString {
            self.color(Color::Red)
        }
        fn yellow(self) -> ColoredString {
            self.color(Color::Yellow)
        }
        fn  blue(self) -> ColoredString {
            self.color(Color::Blue)
        }

        fn color<S: Into<Color>>(self, color: S) -> ColoredString {
            ColoredString {
                fgcolor: Some(color.into()),
                ..self
            }
        }
        fn on_red(self) -> ColoredString {
            self.on_color(Color::Red)
        }
        fn on_yellow(self) -> ColoredString {
            self.on_color(Color::Yellow)
        }
        fn on_blue(self) -> ColoredString {
            self.on_color(Color::Blue)
        }
        fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
            ColoredString {
                bgcolor: Some(color.into()),
                ..self
            }
        }
    }

    impl <'a> Colorize for &'a str {
        fn red(self) -> ColoredString {
            self.color(Color::Red)
        }
        fn yellow(self) -> ColoredString {
            self.color(Color::Yellow)
        }
        fn  blue(self) -> ColoredString {
            self.color(Color::Blue)
        }

        fn color<S: Into<Color>>(self, color: S) -> ColoredString {
            ColoredString {
                fgcolor: Some(color.into()),
                input: String::from(self),
                ..ColoredString::default()
            }
        }
        fn on_red(self) -> ColoredString {
            self.on_color(Color::Red)
        }
        fn on_yellow(self) -> ColoredString {
            self.on_color(Color::Yellow)
        }
        fn on_blue(self) -> ColoredString {
            self.on_color(Color::Blue)
        }
        fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
            ColoredString {
                bgcolor: Some(color.into()),
                input: String::from(self),
                ..ColoredString::default()
            }
        } 
    }

    impl ColoredString {
        fn compute_style(&self) -> String {
            let mut res = String::from("\x1B[");
            let mut has_wrote = false;
            if let Some(ref bgcolor) = self.bgcolor {
                res.push_str(bgcolor.to_bg_str());
                has_wrote = true;
            }
            
            if let Some(ref fgcolor)  = self.fgcolor {
                if has_wrote { res.push(';'); }
                res.push_str(fgcolor.to_fg_str())
            }

            res.push('m');
            res 
        }
    }

    use std::fmt;
    impl fmt::Display for ColoredString  {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let input = &self.input.clone(); // Copy ColoredString 中记录的input字段
            f.write_str(&self.compute_style())?; // write_str,用于将指定的数据记录到底层的缓冲区中
            f.write_str(input)?;
            f.write_str("\x1B[0m")?;
            Ok(())
        }
    }


    #[test]
    fn basic() {
    //     #[derive(Debug)]
    //     enum A {
    //         name,
    //         location(String),
    //     }

    //     let a = A::location(String::from("hello, world"));
    //     match a {
    //         A::name => println!("Name!!"),
    //         A::location(ref s) => println!("lication {}", s),
    //     }
    //     println!("a = {:?}", a);


        let red = "red".red();
        println!("{}", red);

        let yellow = "yellow".yellow().on_blue();
        println!("{}", yellow);
        
        let blue = "blue".blue();
        println!("{}", blue);

        let red = "red".color("red");
        println!("{}", red);

        let yellow = "yellow".on_color("yellow");
        println!("{}", yellow);

        let red = "hello, world".on_color(String::from("red"));
        println!("{}", red);
    }
}