/// 语言处理器的第一个组成部分是词法分析器(lexical ananlyzer).
/// 程序的源代码最初只是一长串字符串。从内部来看，源代码中的换行也能用专门的（不可见)
/// 换行符表示，因此整个源代码是一种相连的长字符串。这样的长字符串很难处理，语言处理器
/// 通常会首先将字符串中的字符以单词为单位分组，切割成多个子字符串，这就是词法分析。
/// 
/// 
/// 词法分析器将把程序源代码视作字符串，并把它分割为若干单词。分割后得到的单词并不是简单
/// 地使用String对象表示，而是使用Token 对象，这种对象处理了记录该单词对应的字符串，
/// 还会保存单词的类型，单词所处位置的行号等信息。
/// 
/// 实际的单词是Token类的子类对象。Token类根据单词的类型，又定义了不同的子类，Stone语言
/// 含有标识符，整型字面量和字符串字面量这三种类型的单词，每种单词都定义了对应的Token类的
/// 子类。每种子类都覆盖了Token类的is_identifier(如果是标识符则为真)， is_number(如果是整型
/// 字面量则为真)及is_string(如果是字符串字面量则为真)方法，并根据具体类型返回相应的值。
/// 
/// Stone语言还定义了一个特别的单词，token.eof 用于表示程序结束。
/// token.eol用于表示换行符，

pub trait Token {
    fn get_line_number(&self) -> i32 {
        -1
    }
    fn is_identifier(&self) -> bool { false }
    fn is_number(&self) -> bool { false }
    fn is_string(&self) -> bool { false }
    fn get_number(&self) -> i32 {
        panic!("not number token");
    }
    fn get_text(&self) -> String { String::from("") }
}

use std::io::{ self, Read, Write, ErrorKind };

enum ParseError {
    TokenEof(i32),
    TokenEol(String),
}

const TOKEN_EOF : i32 = -1;         // end of file
const EOL : &'static str = "\\n";   // end of line
static REGEPAT : &'static str 
    = "\\s*( (//.*) | ([0-9]+) | (\"(\\\\\"|\\\\\\\\|\\\\n|[^\"])*\") | ([A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\\|\\||\\p{Punct}) )?";

// const pat1 : &str = "([0-9]+)";
// const pat2 : &str = "(\"(\\\\\"|\\\\\\\\|\\\\n|[^\n])*\")";
// const pat3 : &str = "[]";
struct Lexer<R : Read> {
    has_more : bool,
    reader : R,
    queue : Vec<Box<dyn Token>>,
}

impl <R : Read> Lexer<R> {
    pub fn read(&mut self) -> Result<Box<dyn Token>, ParseError> {
        unimplemented!()
    }
    pub fn peek(&self, _i: i32) -> Result<Box<dyn Token>, ParseError> {
        unimplemented!()
    }

    fn fill_queue(&self, _i : i32) -> Result<bool, ParseError> {
        unimplemented!()
    }
    fn read_line(&self) -> Result<(), ParseError> {
        unimplemented!()
    }

    fn add_token(&mut self, line_no : i32, matcher : regex::Captures) -> Result<(), ParseError> {
        let m = matcher.get(1);
        
        unimplemented!()
    }
    fn to_string_literal(s : String) {
        unimplemented!()
    }
}

struct Num {
    value : i32,
    line_number : i32,
}

impl Num {
    pub fn new(value: i32, line_number : i32) -> Self {
        Self{
            value,
            line_number,
        }
    } 
}

impl Token for Num {
    fn is_number(&self) -> bool {
        true
    }
    fn get_text(&self) -> String {
        self.value.to_string()
    }
    fn get_number(&self) -> i32 {
        self.value
    }
}
struct Id {
    text: String, 
    line_number : i32,
}

impl Id {
    pub fn new(text: String, line_number: i32) -> Self {
        Self {
            text,
            line_number,
        }
    }
}

impl Token for Id {
    fn is_identifier(&self) -> bool {
        true
    }
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

struct Str {
    literal : String,
    line_number : i32,
}

impl Str {
    pub fn new(literal : String, line_number : i32) -> Self {
        Self {
            literal,
            line_number,
        }
    }
}

impl Token for Str {
    fn is_string(&self) -> bool {
        true
    }
    fn get_text(&self) -> String {
        self.literal.clone()
    }
}