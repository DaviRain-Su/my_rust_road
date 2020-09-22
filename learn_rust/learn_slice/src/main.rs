
// 1. 字符串slice是String中的一部分值的引用
// 2. 字符串字面值就是slice
// 3. 其他类型slice
fn main() {
    println!("Hello, world!");
    let s = String::from("hello, world!");

    let _h = &s[0..5]; // [);
    let _h = &s[0..=4];
    let _h = &s[..=4];
    let h = &s[..5];
    println!("h = {}", h);


    let w = &s[7..];
    let w =&s[7..=10];
    let w = &s[7..];
    println!("w = {}", w);

    // let ss = String::from("你好");
    // let w1 = &ss[0..1];
    // println!("w1 = {}", w1);


    // 字面值
    let s3 = "hello, world";
    println!("s3 = {}", s3);

    let a = [1, 2, 3, 4, 5];
    let sss = &a[0..3];
    println!("sss = {:?}", sss);
}
