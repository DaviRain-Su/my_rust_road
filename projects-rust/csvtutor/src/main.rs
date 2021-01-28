// This makes the csv crate accessible to your program.
extern crate csv;

// import the standard library's IO module so we can read from stdin. 
use std::io;
// The 'main' function is where your program starts executing. 
fn main() {
    // create a scv parser that reads data from stdin. 
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // loop over each recors. 
    for result in rdr.records() {
        // An error may occurs, so abort the program in an unfriendly way. 
        let record = result.expect("a SCV record");

        println!("{:?}", record);
    }
    // println!("Hello, world!");
}
