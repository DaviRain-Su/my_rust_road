fn main() {
    println!("Hello, world!");
    // let ref_s = dangle();

}

// 悬垂引用
// fn dangle() -> &'static str {
    // let s = String::from("hello");
    // &s  // cannot return reference to local variable `s`
// }

// 1. 在任意给定时间，有了可变引用之后，不能再有不可变引用
// 2 引用必须有用