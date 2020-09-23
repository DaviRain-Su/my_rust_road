//1 rust语言中错误分为了两个类别：可恢复错误和不可恢复的错误
//1.1 可恢复的错误代表向用户报告错误和重试操作是合理的情况，例如没有找到文件，
// rust 中使用Result<T, E>来实现
//1.2 不可恢复错误是bug的同义词，如尝试访问超过数组结尾的位置，rust通过panic！来实现
//2. panic
//3. 使用RUST_BACKTRACE=1
//4. Result<T, E>
// enum Result<T, E {
//   Ok(T),
//   Err(E),
// }

//5 简写
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    
    let f = File::open("src/main.rs").unwrap();
    let f = File::open("src/main.rs").expect("cannot to open file");
    let f = File::open("src/main.rs");

    let mut f = match f {
        Ok(file) => file,
        Err(err) => panic!("error : {:?}", err),
    };

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    println!("buffer = {}", buffer);
    Ok(())
    
    // panic!("hello, world");
    // println!("Hello, world!");
// RUST_BACKTRACE=1
//     stack backtrace:
//    0: std::panicking::begin_panic
//              at /Users/suyinrong/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:505
//    1: learn_err1::main
//              at ./src/main.rs:11
//    2: core::ops::function::FnOnce::call_once
//              at /Users/suyinrong/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227
// note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}
