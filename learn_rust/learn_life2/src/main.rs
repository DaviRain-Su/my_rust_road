
// 函数中的生命周期

// fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
fn longest<'a> (x : &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y 
    }
}

fn get_str<'a>(x: &'a str, _y: &str) -> &'a str {
    x 
}

fn a_str<'a>(_x: &'a str, _y: &'a str) -> &'a str {
    // let r = String::from("abd");
    // r.as_ref() 
    _x  
}

// error[E0515]: cannot return value referencing local variable `r`
//   --> src/main.rs:19:5
//    |
// 19 |     r.as_ref()
//    |     -^^^^^^^^^
//    |     |
//    |     returns a value referencing data owned by the current function
//    |     `r` is borrowed here



fn main() {

    let s1 = String::from("hello, world");
    let s2 = String::from("world");
    let s3 = longest(&s1, &s2);
    println!("s3 = {}", s3);

    let ss = get_str(s1.as_ref(),  s2.as_ref());
    println!("ss = {:?}", ss);
    let ss2 = a_str(s1.as_ref(), s2.as_ref());
    println!("ss2 = {}", ss2);
    println!("Hello, world!");
}
