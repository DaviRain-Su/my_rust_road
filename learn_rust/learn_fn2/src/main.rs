fn main() {
    println!("Hello, world!");
    let s1 = String::from("hello, world");
    let s1 = takes_ownership(s1);
    println!("s1 = {}", s1);

    let s2 = 3;
    makes_copy(s2);
    println!("s2 = {}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("some_string = {}", some_string);
    some_string
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}