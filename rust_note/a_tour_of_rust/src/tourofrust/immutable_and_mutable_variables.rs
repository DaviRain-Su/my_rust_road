/// # 可变性与不可变性
/// 
/// Rust中的值分为两种类型：
/// - 可变的，编译器允许对变量进行读取和写入， 使用mut关键字表示
/// - 不可变的，编译器只允许对变量进行读取
pub fn immutable_and_mutable_variables_example() {
    // for mutable
    let mut x = 42;
    println!("change before x = {}", x);
    x = 13;
    println!("change after x = {}", x);

    // for immutable
    let x = 34;
    println!("change before x = {}", x);
    // x = 23;
    println!("change after x = {}", x);
}