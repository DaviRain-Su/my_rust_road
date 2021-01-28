#![feature(conservative_impl_trait)]
extern crate futures;
extern crate futures_cpupool;

use std::io;
use futures::Future;
use futures_cpupool::CpuPool;


// This implementation returns a boxed future
fn check_prime_boxed(n: u64) -> Box<dyn Future<Item = bool, Error = io::Error>> {
    for i in 2..n {
        if n % i == 0 {
            return Box::new(futures::future::ok(false));
        }
    }
    Box::new(futures::future::ok(true))
}
// This returns a futures using impl trait
fn check_prime_impl_trait(n: u64) -> impl Future<Item = bool, Error = io::Error> {
    for i in 2..n {
        if n % i == 0 {
            return Box::new(futures::future::ok(false));
        }
    }
    Box::new(futures::future::ok(true))
}

// This does not return a future
fn check_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 { return false; }
    }
    true
}

fn main() {
    // println!("Hello, world!");
    let input : u64 = 58466453;
    println!("Right before first call");
    let res_one = check_prime_boxed(input);
    println!("Called check_prime_boxed");
    let res_two = check_prime_impl_trait(input);
    println!("Called check_prime_impl_trait");
    println!("Results are {} and {}", res_one.wait().unwrap(), res_two.wait().unwrap());

    let thread_pool = CpuPool::new(4);
    let res_three = thread_pool.spawn_fn(move || {
        let temp = check_prime(input);
        let result: Result<bool, ()> = Ok(temp);
        result
    });
    println!("Called check_prime in other thread");
    println!("Result from the last call: {}", res_three.wait().unwrap());

}

