#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    Black, // 0
    Red,   // 1
    Green, // 2
    Yellow, // 3
    Blue,   // 4
    Pink,   // 5
    Indigo, // 6
    White // 7
}

impl  Color {
    fn to_fg_str(&self) -> &str { // 前景色ANSI码的获得
        match *self { // 这里的*self 并不会转移所有权
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Pink => "35",
            Color::Indigo => "36",
            Color::White => "37",
        }
    }
    fn to_bg_str(&self) -> &str { // 背景色ANSI码的获得
        match *self {
            Color::Black => "40",
            Color::Red => "41",
            Color::Green => "42",
            Color::Yellow => "43",
            Color::Blue => "44",
            Color::Pink => "45",
            Color::Indigo => "46",
            Color::White => "47",
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
            "black" => Ok(Color::Black), // 0
            "red" => Ok(Color::Red), // 1
            "green" => Ok(Color::Green), // 2
            "yellow" => Ok(Color::Yellow), // 3
            "blue" => Ok(Color::Blue), // 4
            "pink" => Ok(Color::Pink), // 5
            "indigo" => Ok(Color::Indigo), // 6
            "white" => Ok(Color::White), // 7
            _ => Err(()),
        }
    }
}

// 增减了color, on_color方法，使用这两个方法，就可以通过字符串来设置终端文本的颜色
// color, on_color泛型方法中使用了triat限定<S: Into<Color>>, 这是因为
// Color实现了From, 所以对于String, &'a str 类型字符串均可通过into方法
// 转换为Color. 
trait Colorize {
    fn black(self) -> ColoredString;
    fn red(self) -> ColoredString;
    fn yellow(self) -> ColoredString;
    fn green(self) -> ColoredString;
    fn blue(self) -> ColoredString;
    fn pink(self) -> ColoredString;
    fn indigo(self) -> ColoredString;
    fn white(self) -> ColoredString;
    // 这个color中的泛型S被限定为了Into<Color>, 这样只要实现了Into<Color>的类型
    // 才可以使用, 而前面为String, 和&'a str都都实现了From，
    // 而只要实现了From trait， Into trait就会被自动实现
    fn color<S: Into<Color>>(self, color: S) -> ColoredString;
    fn on_black(self) -> ColoredString;
    fn on_red(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
    fn on_green(self) -> ColoredString;
    fn on_blue(self) -> ColoredString;
    fn on_pink(self) -> ColoredString;
    fn on_indigo(self) -> ColoredString;
    fn on_white(self) -> ColoredString;
    // 这on_color中的泛型S被限定为了Into<Color>, 这样只要实现了Into<Color>的类型
    // 才可以使用        
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
}

impl Colorize for ColoredString {
    fn black(self) -> ColoredString {
        self.color(Color::Black)
    }
    fn red(self) -> ColoredString {
        self.color(Color::Red)
    }
    fn yellow(self) -> ColoredString {
        self.color(Color::Yellow)
    }
    fn green(self) -> ColoredString {
        self.color(Color::Green)
    }
    fn  blue(self) -> ColoredString {
        self.color(Color::Blue)
    }
    fn pink(self) -> ColoredString {
        self.color(Color::Pink)
    }
    fn indigo(self) -> ColoredString {
        self.color(Color::Indigo)
    }
    fn white(self) -> ColoredString {
        self.color(Color::White)
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            ..self
        }
    }
    fn on_black(self) -> ColoredString {
        self.on_color(Color::Black)
    }
    fn on_red(self) -> ColoredString {
        self.on_color(Color::Red)
    }
    fn on_yellow(self) -> ColoredString {
        self.on_color(Color::Yellow)
    }
    fn on_green(self) -> ColoredString {
        self.on_color(Color::Green)
    }
    fn on_blue(self) -> ColoredString {
        self.on_color(Color::Blue)
    }
    fn on_pink(self) -> ColoredString {
        self.on_color(Color::Pink)
    }
    fn on_indigo(self) -> ColoredString {
        self.on_color(Color::Indigo)
    }
    fn on_white(self) -> ColoredString {
        self.on_color(Color::White)
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            ..self
        }
    }
}

impl <'a> Colorize for &'a str {

    fn black(self) -> ColoredString {
        self.color(Color::Black)
    }
    fn red(self) -> ColoredString {
        self.color(Color::Red)
    }
    fn yellow(self) -> ColoredString {
        self.color(Color::Yellow)
    }
    fn green(self) -> ColoredString {
        self.color(Color::Green)
    }
    fn  blue(self) -> ColoredString {
        self.color(Color::Blue)
    }
    fn pink(self) -> ColoredString {
        self.color(Color::Pink)
    }
    fn indigo(self) -> ColoredString {
        self.color(Color::Indigo)
    }
    fn white(self) -> ColoredString {
        self.color(Color::White)
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
    fn on_black(self) -> ColoredString {
        self.on_color(Color::Black)
    }
    fn on_red(self) -> ColoredString {
        self.on_color(Color::Red)
    }
    fn on_yellow(self) -> ColoredString {
        self.on_color(Color::Yellow)
    }
    fn on_green(self) -> ColoredString {
        self.on_color(Color::Green)
    }
    fn on_blue(self) -> ColoredString {
        self.on_color(Color::Blue)
    }
    fn on_pink(self) -> ColoredString {
        self.on_color(Color::Pink)
    }
    fn on_indigo(self) -> ColoredString {
        self.on_color(Color::Indigo)
    }
    fn on_white(self) -> ColoredString {
        self.on_color(Color::White)
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
/*
    let black = "black".black().on_blue();
    println!("{}", black);

    let red = "red".red().on_green();
    println!("{}", red);

    let green = "green".green().on_red();
    println!("{}", green);

    let yellow = "yellow".yellow().on_black();
    println!("{}", yellow);
    
    let blue = "blue".blue().on_red();
    println!("{}", blue);

    let pink = "pink".pink().on_black();
    println!("{}", pink);

    let indigo = "indigo".indigo().on_black();
    println!("{}", indigo);

    let white = "white".white().on_black();
    println!("{}", white);

    let red = "red".color("red").on_black();
    println!("{}", red);

    */
    let yellow = "yellow".on_color("yellow").black();
    println!("{}", yellow);

    let red = "hello, world".on_color(String::from("red"));
    println!("{}", red);
}