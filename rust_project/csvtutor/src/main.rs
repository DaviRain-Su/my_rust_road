// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::io;
use std::error::Error;
use std::process;

// The 'main' function is where your program starts executing.
fn main() {
     if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // This is effectively the same code as our 'match' in the
        // previous example. In other words, '?' is syntactic sugar.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}