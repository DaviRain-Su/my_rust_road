extern crate chapter4;
fn main() {
    chapter4::davirain::var_scope();
    // println!("Hello, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s2  = {}, s1 = {}", s2, s1);

}
