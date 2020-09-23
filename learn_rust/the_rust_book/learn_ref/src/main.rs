

// fn main() {
//     let s1 = gives_ownership();
//     println!("s1 = {}", s1);    

//     let s2 = String::from("hello");
//     println!("s2 = {}", s2);

//     let s3 = takes_and_gives_back(s2);
//     println!("s3 = {}", s3);
//     // println!("s2 = {}", s2); // value borrowd moved here

//     println!("Hello, world!");
// }

// fn gives_ownership() -> String {
//     String::from("hello")
// }

// fn takes_and_gives_back(some_thing : String) -> String {
//     println!("some thing = {}", some_thing);
//     some_thing
// }

// 引用 :  & 
// 创建了一个指向值的引用，但是并不拥有这个值。因为不拥有这个值，
// 当引用离开其值指向的作用域后也不会被丢弃
// 借用: &mut 
fn main() {
    let mut s1 = String::from("hello, world");

    let s = &s1;
    let len = calcute_length(s);
    println!("len = {}", len);
    println!("s1 = {}", s1);

    // let mut s2 = String::from("hello");
    let ms = &mut s1;
    modify_s(ms);

    // println!("ms = {}", ms);
    // println!("s1 = {}", s1);
    // println!("s = {}", s);
    {
        let mut s1 = String::from("hello");

        let r1 = &s1; 
        let r2 = &s1;
        println!("r1 = {}, r2 = {}", r1, r2);
        let r3 = &mut s1;
        println!("r3 = {}", r3);
        // println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);
        // println!("r1 =  {}, r2 = {}", r1, r2);
    
    }
}

// 引用
fn calcute_length(s : &String) -> usize {
    s.len()
}

// 借用
fn modify_s(s: &mut String) {
    s.push('3');
    s.push_str("!!!!!!!!");
}