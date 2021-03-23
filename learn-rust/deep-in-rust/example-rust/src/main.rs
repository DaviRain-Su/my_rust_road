#![feature(box_syntax)]

use chrono::{DateTime, Utc};
fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    // println!("Hello, world!");
}

#[test]
fn test_move_type() {
    let mut s = String::new();
    s.push_str("hello world");
    println!("string ---> {}", s);
}

// #[test]
// fn test_move_type_2() {
//     let s = String::from("hello");
//     let s1 = s; // value moved here
//     println!("s = {}", s); // value borrowed here after move
// }


#[test]
fn test_clone_type() {
    let s = String::from("hello world");
    let _s1 = s.clone();
    println!("s = {}", s);
}

// move type 
fn create(args: &str) -> String {
    String::from(args)
}

fn consume(s: String) {
    println!("{}", s);
}

#[test] 
fn test_create_consume() {
    let s = create("hello world");
    consume(s);
}

#[derive(Debug, Clone, Copy)]
struct Foo {
    data: i32,
}

// impl Copy for Foo {}
// impl Clone for Foo {
//     fn clone(&self) -> Self {
//         Self {
//             data: self.data,
//         }
//     }
// }


#[test]
fn test_foo() {
    let foo = Foo{ data: 32};
    // println!("foo = {}", foo);
    let foo_1 = foo;
    println!("foo {:?}, foo_1 = {:?}", foo, foo_1);
    let p = Box::new(Foo{data: 23});
    let p1: Box<Foo> = box Foo{data: 12};

    println!("foo.data = {}", p.data);
    println!("p1.data = {}", p1.data);
}

use std::ops::Drop;

struct D(i32);
impl Drop for D {
    fn drop(&mut self) {
        println!("destruct {}", self.0);
    }
}

#[test]
fn test_struct_drop() {
    let _x = D(1);
    println!("construct 1");
    {
        let _y = D(2);
        println!("construct 2");
        println!("exit inner scope");
    }
    println!("exit main function");
}