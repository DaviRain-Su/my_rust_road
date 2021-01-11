// use std::io;
// use std::thread::spawn;
// use rayon::prelude::*;
// use std::{cmp::Ordering, sync::Arc};
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::sync::atomic::AtomicIsize;
#[macro_use] extern crate lazy_static;

static  CONUT : AtomicIsize = AtomicIsize::new(0);

// lazy_static! {
static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
// }

static mut NUM: u8 = 0;

fn main() {
    // let _arr = vec![0; 10];
    // let arr: Vec<i32> = (0..100000000).into_iter().collect();
    // println!("arr = {:?}", arr);
    // fn _hello() {
    //     println!("hello");
    // }
    // fn _world() {
    //     println!("world");
    // }
    // let sum = Arc::new(Mutex::new(0));
    // arr.par_iter().for_each(|val| {
    //     // println!("val = {}", val);
    //     // sum += val;
    //     *sum.lock().unwrap() += val;
    // });
    // println!("sum = {:?}", sum);

    // let fn2 = |a: i32, b: i32| a * b;
    // let fn1 = |a: i32, b: i32| a + b;
    // // let (v1, v2) = rayon::join(|| hello(), || world());
    // let (a, b) = (3, 4);
    // let (v1, v2) = rayon::join(|| fn1(a, b), || fn2(a, b));
    // println!("v1 = {:?}, v2 = {:?}", v1, v2);
    // println!("Hello, world!");

    // let arr: Vec<i32> = (0..100).into_iter().collect();
    let mut ret = vec![];
    for i in 0..10 {
        let handle = std::thread::spawn(move || {
           CONUT.fetch_add(1 , Ordering::SeqCst); 
           HOSTNAME.lock().unwrap().push_str(&format!("{}", i));
        });
        ret.push(handle);
    }

    for val in ret {
        val.join().unwrap();
    }

    unsafe {
        NUM += 1;
    }
    unsafe {
        println!("NUM = {:?}", NUM);
    }
    

    println!("Count = {:?}", CONUT);
    println!("HOSTNAME = {:?}", HOSTNAME.lock());
}

// fn process_files(filenames: Vec<String>) -> io::Result<()> {
//     for document in filenames {
//         // let text = load(&document)?; // read source file
//         // let result = process(text); // compute static
//         // save(&document, result)?; // write output file
//     }
//     Ok(())
// }

// fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {

//     const NTHREADS: usize = 8;

//     let worklists = split_vec_into_chunks(filenames, NTHREADS);

//     let mut thread_handles = vec![];

//     for worklist in worklists {
//         thread_handles.push(
//             spawn(move || process_files(worklist))
//         );
//     }

//     for handle in thread_handles {
//         handle.join().unwrap();
//     }

//     Ok(())
// }
