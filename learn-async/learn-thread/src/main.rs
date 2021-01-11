use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, vec};

fn main() {
    let arr = vec![1, 2, 3];
    let arc_arr = Arc::new(Mutex::new(arr));
    let arr1 = arc_arr.clone();
    println!("Print child vec");
    let child = thread::Builder::new()
        .name("child1".to_string())
        .spawn(move || {
            for val in arr1.lock().unwrap().iter_mut() {
                *val += 1;
            }
            for v in arr1.lock().unwrap().iter() {
                println!("cv: {}", v);
            }
        })
        .unwrap();

    let _ = child.join().unwrap();

    println!("Print parent vec");
    for v in arc_arr.lock().unwrap().iter() {
        println!("pv: {}", v);
    }

    test_thread();
    test_thread_2();
    test_thread_3();
}

fn test_thread() {
    let t = thread::Builder::new()
        .name("child2".to_string())
        .spawn(move || {
            println!("enter child thread.");
            thread::park();
            println!("resume child thread.");
        })
        .unwrap();
    println!("spawn a thread");
    // thread::sleep(Duration::new(5, 0));
    t.thread().unpark();
    let _ = t.join();
    println!("child thread finished");
}

fn test_thread_2() {
    let mut health = 12;
    thread::spawn(move || { // mutable borrow occurs here
        health *= 2; // firs borrow occurs due to use of 'health' in closure
        println!("c-health = {}", health);
    });
    // argument requires that 'helth' is borrowed for 'static'
    println!("{}", health);// immutable borrow occurs here.
}

fn test_thread_3() {
    let arr = vec![1, 2, 3];
    let mut arr_1 = arr.clone();
    let ret = thread::spawn(move || {
        arr_1.push(1);
        println!("c - arr_1: {:?}", arr_1);
    });
    let _ = ret.join();
    println!("arr: {:?}", arr);
}