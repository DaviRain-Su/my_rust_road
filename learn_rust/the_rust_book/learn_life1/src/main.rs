// 1 Rust 中每一个引用都有生命周期，也就是引用保持有效的作用域，
// 大部分时候生命周期是隐含并可以推断的，正如大部分的时候类型可以推断一样。
//
// 2 生命周期的主要目标是避免悬垂引用
// 
// 3 Rust 编译器使用借用检查器来检查生命周期是否有效。

fn main() {
    // let r;              // ------------------+------------'a
    //                           //                   +
    //                           //                   |
    // {                         //-------+----'b     |
    //     let x = 5;       //       |           |
    //     r = &x;               //-------+           |
    // }                         //                   |
    // println!("r = {}", r);    //                   |
    //                           //-------------------+

//     error[E0597]: `x` does not live long enough
//   --> src/main.rs:12:13
//    |
// 12 |         r = &x;
//    |             ^^ borrowed value does not live long enough
// 13 |     }
//    |     - `x` dropped here while still borrowed
// 14 |     println!("r = {}", r);
//    |                        - borrow later used here

    let r;
    let x = 5;
    r = &x;
    println!("r = {}", r);
    println!("Hello, world!");
}
