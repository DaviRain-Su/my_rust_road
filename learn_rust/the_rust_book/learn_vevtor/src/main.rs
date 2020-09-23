// 1 创建空的vector Vec<T>
// 2 创建包含初始值的vector
// 3 丢弃vector
// 4 读取元素
// 5 更新
// 6 遍历
// 7 使用枚举
 
fn main() {
    // 1 
    let mut vec = Vec::new();
    vec.push(3);

    // 2
    let _v = vec![1, 2, 3, 4];

    // 3 
    {
        let _v1 = vec![1, 2, 34];
    }

    // 4 
    // 常规的直接索引
    let one : &i32 = &_v[0];
    println!("one = {}", one);
    println!("one = {}", *one);
    // 推荐的方法 get 方法
    match _v.get(4) {
        Some(value) => {
            println!("value = {}", value);
        }
        _ => println!("None"),
    }
    // 5 
    let mut v2 = Vec::new();
    v2.push(3);
    v2.push(2);
    v2.push(1);

    println!("v2 = {:?}", v2);

    // 6 
    // 不可变的遍历
    for i in &v2 {
        println!("i = {}", i);
    }

    // 可变的遍历
    for i in v2.iter_mut() {
        *i += 1;
    }
    println!("v2 = {:?}", v2);
    
    // 7 
    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }

    let c = vec![
        Context::Text(String::from("String")),
        Context::Int(-1),
        Context::Float(0.01),
    ];

    for i in c.iter() {
        println!("i = {:?}", i);
    }

    // 8 补充
    let mut v = vec![1, 2, 3, 4, 5];

    let first  = &v[0]; // immutable borrow occurs here
    v.push(4); // mutable borrow occurs here
    // println!("first = {}", first); // immutable borrow later used here

    println!("Hello, world!");
}
