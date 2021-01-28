// 1 创建一个空的String
// 2 通过字面量创建一个Stirng
// 2.1 使用String::from()
// 2.2 使用str的方式
// 3. 更新字符串
// 3.1 push_str
// 3.2 push 
// 3.3 使用+ 合并字符串
// 4 使用format
// 5 Stirng索引
// 6 遍历
// 6.1 chars
// 6.2 bytes

// 
fn main() {
    //1
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);
    
    // 2.1
    let s1 = String::from("hello, world");
    println!("s1 = {}", s1);
    
    // 2.2
    let s1 = "hello, world".to_string();
    println!("s1 = {}", s1);

    // 3
    let mut s2 = String::from("hello");
    s2.push_str(", world");
    println!("s2 = {}", s2);

    //3.1
    let ss = " !".to_string();
    s2.push_str(&ss);
    println!("s2 = {}", s2);
    println!("ss = {}", ss);

    //3.2
    let mut s2 = String::from("tea");
    s2.push('m');
    println!("s2 = {}", s2);
    s2.push('s');
    println!("s2 = {}", s2);

    // 4 +
    let s1 = String::from("hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1);

    // format
    let s1 = String::from("tic");
    let s2 = String::from("123");
    let s3 = String::from("345");
    let s4 = format!("{}-{}-{}", s1, s2, s3); // format和println宏是一样的
    println!("s4 = {}", s4);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

    // 索引
    let s4 = String::from("hello");
    // let s41 = s4[0]; // `std::string::String` cannot be indexed by `{integer}`
    println!("s4 len = {}", s4.len());
    println!("{}", &s4[..1]);
    println!("{}", &s4[1..2]);
    println!("{}", &s4[2..3]);
    println!("{}", &s4[3..4]);
    println!("{}", &s4[4..5]);



    let s4 = String::from("你好");
    // let s5 = &s4[0..2]; // thread 'main' panicked at 'byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`',
    let s5 = &s4[0..3];
    println!("s5 = {}", s5);
    println!("s4 len = {}", s4.len());

    // 遍历 chars
    for c in s4.chars() {
        println!("{}", c);
    }

    // Bytes
    for c in s4.bytes() {
        println!("{}", c);
    }

    println!("Hello, world!");
}
