extern crate pat_solution;
use std::io;

use pat_solution::pat_1002::write_the_number;

fn main() {

    let mut input_string = String::new();

    io::stdin().read_line(&mut input_string)
        .expect("Failed to read line");

    let n = input_string.trim().to_string();
    // let n = "1234567890987654321123456789".to_string();
    let ret = write_the_number(n);
    // println!("ret = {:?}", ret);
    for i in 0..ret.len() {
        if i == ret.len() - 1 {
            print!("{}", ret.get(i).unwrap());
        }else {
            print!("{} ", ret.get(i).unwrap());
        }
    }
}
