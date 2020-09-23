// 1 HashMap<K,V>
// 2 创建HashMap
// 3 读取
// 4 遍历
// 5 更新
use std::collections::HashMap;
fn main() {
    let mut score : HashMap<String, i32> = HashMap::new();
    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Red"), 20);
    println!("score = {:?}", score);


    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<&String, &i32> = keys.iter().zip(values.iter()).collect();
    
    // 索引
    // let k = String::from("Blue");
    let k = String::from("123");
    let v = scores.get(&k); // get 返回的是一个Option
    if let Some(v) = scores.get(&k) {
        println!("v = {}", v);
    }
    match v {
        Some(v) => println!("v = {}", v),
        None => println!("None"),
    }

    // let v = scores.get(&k).unwrap();
    // println!("v = {:?}", v);
    
    // 遍历
    for (&key, &value) in scores.iter() {
        println!("key = {}, value = {}", key, value);
    }

    println!("__________________________");
    for (key, value) in score.iter_mut() {
        println!("key = {}, value = {}", key, value);
        *value = 23;
    }
    println!("__________________________");

    for (key, value) in score.iter() {
        println!("key = {}, value = {}", key, value);
    }

    // 直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("one"), 3);
    println!("ss = {:?}", ss);
    
    //键不存在的时候插入
    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    ss1.insert(String::from("three"), 3);

    ss1.entry("one".to_string()).or_insert(3);
    println!("ss1 = {:?}", ss1);

    // 根据旧值来更新一个值
    let test = "hello, world wonderful world";
    let mut map = HashMap::new();
    for word in test.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map = {:?}", map);

    println!("Hello, world!");
}
