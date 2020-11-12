// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::io;

// The 'main' function is where your program starts executing.
fn main() {
    // Create a CSV parser that reads data from stdin.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Loop over each record.
    for result in rdr.records() {
        // Examine our Result.
        // If there was no problem, print the record.
        // Otherwise, print the error message and quit the problem.
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading CSV from <stdin>:\n{}", err);
                std::process::exit(1);
            }
        }
    }
}
