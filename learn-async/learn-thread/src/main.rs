use std::{thread, vec};
use std::sync::{Arc, Mutex};

fn main() {
    let arr = vec![1, 2, 3];
    let arc_arr = Arc::new(Mutex::new(arr));
    let arr1 = arc_arr.clone();
    println!("Print child vec");
    let child = thread::spawn(move || {
        for val in arr1.lock().unwrap().iter_mut() {
            *val += 1;
        }
        for v in arr1.lock().unwrap().iter() {
            println!("cv: {}", v);
        }
    });
    let _ = child.join().unwrap();

    println!("Print parent vec");
    for v in arc_arr.lock().unwrap().iter() {
        println!("pv: {}", v);
    }
}
