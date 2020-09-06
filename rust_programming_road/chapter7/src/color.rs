pub trait Colorize {
    // 关联常量和关联类型类似，由实现该triat的类型来指定常量的值，
    // 也可以在trait中指定默认值，如下
    //
    // 使用关联常量的好处
    //
    // 可读性和维护性更高，相比直接使用数值
    // 
    // 使用关联常量需要注意的有
    // 
    // 使用关联常量，要注意常量名要都是大写，否则编译器会输出警告
    // 并且在trait中要明确标注好常量的类型，
    // 因为此处编译器无法推断常量的具体类型
    //
    const FG_RED: &'static str = "31"; //关联常量
    const BG_YELLOW: &'static str = "43"; // 关联常量
    fn red(self) -> ColoredString; // 用于设置前景色
    fn on_yellow(self) -> ColoredString; // 用于设置背景色
}

#[derive(Debug)]
pub struct ColoredString {
    input : String, // raw string
    fgcolor : String, // 前景色
    bgcolor : String, // 背景色
}


// Rust 标准库std::default模块中提供了一个叫做Deault trait来实现，默认构造。
// 也就是C++语言中的构造函数，这里只是在类比于C++的构造函数. 
// 这里使用Default 可以为ColoredString 提供默认值。
// Default 已经在std::prelude::v1 模块中导入了，所以这里就可以直接使用
// 而不需要显式地调用Default. 
// Rust 为内置的大部分类型实现了Default. 
// 所以下面使用了String::Default方法来设置了String类型的默认值
impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: String::default(),
            bgcolor: String::default(),
        }
    }
}

// 为 ColoredString 实现 Colorize
// 
// 这里为什么也为ColoredString 也实现了Colorize 
// 
// 如果只需要为字符串设置前景色或背景色中的某一种，那么只需要为&'a str,
// 实现Colorize, 就能满足"hello".red() 的调用。
// 
// 但是要是想这样 "Hello".red().on_yellow()这样的调用通过
// 链式调用来同时设置前景色和背景色，就需要为ColoredString也实现Colorize, 
// 因为在第一次调用之后，返回的是ColoredString类型的实例，ColoredString 必须
// 实现Colorize才能满足这样的链式调用。
// 
//
// 注意关联常量的写法，因为是为ColoredString类型来实现的Colorize, 所以调用
// 关联常量也必须以类型名为前缀，即ColoredString::FG_RED， ColoredString::BG_YELLOW. 
//  
//
impl <'a> Colorize for ColoredString {
    // 当实现red 方法时， 只需要设置前景色 fgcolor, 而另外两个值却不知道，原始文本可能是任意字符串
    // 背景色bgcolor可以设置，也可以不设置。
    fn red(self) -> ColoredString {
        ColoredString{
            fgcolor: String::from(ColoredString::FG_RED),
            ..self // 结构体更新语法，bgcolor, input使用原来的值
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from(ColoredString::BG_YELLOW),
            ..self // 结构体更新语法，fgcolor, input使用原来的
            //  如果有链式调用才会轮到ColoredString类型，所以需要self来保存第一次
            // 调用时的设置
        }
    }
}

// 为&'a str 实现Colorize 
// 从&'a str 构造出 ColoredString
impl <'a> Colorize for &'a str {
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from(ColoredString::FG_RED),
            input: String::from(self),
            ..ColoredString::default() // 这里使用的是ColoredString的默认构造函数，
            // 原因是，第一次调用red或on_yellow方法的是字符串，所以需要调用ColoredString
            // 的默认值来进行第一次默认填充。
        }
    }
    fn on_yellow(self) -> ColoredString {
        ColoredString{
            bgcolor: String::from(ColoredString::BG_YELLOW),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
}

// 为ColoredString实现compute_style
// compute_style 方法是为了组装ANSI序列的前半部分
// 也就是\x1B[43;31m. 
// 前半部分是使用前缀\x1B来定义的ANSI序列的开始，
// 后面紧跟颜色编码43;31， 43和31分别代表黄色的前景色和红色背景色，
// 用分号相隔，结束符m代表颜色控制字符已经设置完毕。
// 
// Q5: 对于ANSI控制字符来说， 前景色和背景色是由相应的代码决定的，和他们的拼接顺序无关系。
// A5: 所以这里最终拼接结果是43;31，先判断背景色，然后是前景色，其实反过来, 31;43也不会
// 影响呈现结果。
//
impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;

        if !self.bgcolor.is_empty() {
            res.push_str(&self.bgcolor);
            has_wrote = true;
        }

        if !self.fgcolor.is_empty() {
            if has_wrote { res.push(';'); }
            res.push_str(&self.fgcolor);
        }
        res.push('m');
        res 
    }
}

// 需要将最终的输出结果拼接出完整的字符串\x1B[43;31mHello\x1b[0m,那就需要
// coloredString 实现Display, 
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
fn basic () {
    let hi = "Hello".red().on_yellow();
    println!("{:?}", hi);

    let hi = "Hello".on_yellow();
    println!("{:?}", hi);

    let hi = "Hello".red();
    println!("{:?}", hi);

    let hi = "Hello".on_yellow().red();
    println!("{:?}", hi);
}
