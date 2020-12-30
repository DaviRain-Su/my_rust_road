use std::collections::LinkedList;

#[test]
fn test_linkedlist() {
    let mut list = LinkedList::new();
    list.push_back('a');
    println!("list = {:?}", list);

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    println!("list2 = {:?}", list2);

    list.append(&mut list2);
    println!("list2 = {:?}", list2);
    for val in list.iter() {
        println!("val = {}", val);
    }

    let mut vec = vec![];
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("vec = {:?}", vec);
    let vec : Vec<i32> = vec.iter().rev().map(|&val| val).collect();
    println!("vec = {:?}", vec);
}