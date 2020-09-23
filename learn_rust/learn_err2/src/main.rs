// 1. 当编写一个函数，但是该函数可能会失败，此时除了了在函数中处理错误外，还可以将
// 错误传给调用者，让调用者决定如何处理错误，这被称为传播错误
// 2 传播错误的简写方式，提倡的方式
// 3 更进一步的简写
// 4 什么时候用panic!, 什么时候使用Result
// 4.1 示例，代码原型，测试用panic!\unwrap\expect
// 4.2 实际项目中应该使用Result
// 5 Option和Result

use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let s = match read_username_from_file() {
        Ok(s) => s,
        Err(err) => format!("{:?}", err),
    };
    println!("{}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 1
    // let f = File::open("hello.txt");
    // let mut f = match f { 
    //     Ok(file) => file,
    //     Err(err) => return Err(err),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(err) => Err(err), 
    // }
    // 2----------------------------
    // let mut f = File::open("hello.txt")?;
    
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;

    // // println!("s = {}", s);
    // Ok(s)

    //3------------------------
    let mut s = String::new();

    File::open("hello.txt")?
        .read_to_string(&mut s)?;
    
    Ok(s)
}