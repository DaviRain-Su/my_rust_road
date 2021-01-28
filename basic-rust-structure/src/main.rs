extern crate basic_rust_structure;

use basic_rust_structure::Stack;
use basic_rust_structure::Queue;
fn main() {
    println!("------------Stack--------------");
    // test stack
    #[derive(Eq, PartialEq, Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct{ a: 5};
    let b = TestStruct{ a: 9};


    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(),None);
    println!("s = {:?}", s);

    s.push(&a);
    s.push(&b);
    println!("s = {:?}",s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
    // println!("Hello, world!");


    // Queue
    println!("------------Queue---------------");

    let mut q = Queue::new();
    println!("q = {:?}", q);
    println!("q.pop empty is {:?}", q.pop());
    q.push(1);
    q.push(2);
    q.push(3);
    println!("q = {:?}", q);
    println!("q.pop is {:?}", q.pop().unwrap());
    println!("q = {:?}", q);
}
