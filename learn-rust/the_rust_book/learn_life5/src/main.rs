// 1 静态生命周期
// 定义方式： &'static
// 其生命周期存活于整个程序期间，所有的字符字面值都有拥有字节的static 生命周期
// let s: &'static str = "hello";

use std::fmt::Display;

fn function<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("ann is {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let s1 = String::from("i am s1");
    let s2 = String::from("i am s2, hello");
    let ann = 1239;
    let r = function(s1.as_ref(), s2.as_ref(), ann);
    println!("r = {}", r);
    println!("Hello, world!");
}
