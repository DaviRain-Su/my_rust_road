// 1. 定义结构体
#[derive(Debug)]
struct User {
    name: String,
    count: String,
    nonce: u64,
    active: bool,
}

fn main() {
    // 2. 创建结构体
    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("19951223"),
        nonce: 1000,
        active: true,
    };

    println!("xiaoming = {:#?}", xiaoming);

    let mut xiaohuang = User {
        name: String::from("xiaoming"),
        count: String::from("19951223"),
        nonce: 1000,
        active: true,
    };

    xiaohuang.nonce = 3000;

    println!("xiaohuang = {:#?}", xiaohuang);

    // 4. 参数名字和字段名字相同时的简写方法
    let name = String::from("xiaoxiao");
    let count = String::from("19951223");
    let nonce = 34000;
    let active = false;


    // let use1 = User {
    //     name : name,
    //     count : count,
    //     nonce: nonce, 
    //     active: active,
    // };
    let use1 = User {
        name,
        count,
        nonce, 
        active,
    };
    println!("user1 = {:#?}", use1);

    // 5 从其他结构体创建实例
    let user2 = User {
        name: String::from("user2"),
        ..use1
    };
    println!("user2 = {:#?}", user2);

    // 6 元组结构体
    // 字段没有名字
    // 圆括号
    #[derive(Debug)]    
    struct Point(i32, i32);

    let a = Point(10, 20);
    let b = Point(30, 23);

    println!("a = {:#?}", a);
    println!("b = {:#?}", b);

    // 7 元组结构体，没有任何字段的结构体
    #[derive(Debug)]
    struct A {};

    let a = A{};
    println!("a = {:?}", a);
    
    println!("Hello, world!");
}




