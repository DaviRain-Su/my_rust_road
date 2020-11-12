// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process;
// use std::fs::File;
use std::env;
use std::collections::HashMap;

// The 'main' function is where your program starts executing.
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// This introduces a type alias so that  we can conveniently reference our
// record type.
type Record = HashMap<String, String>;

fn run() -> Result<(), Box<dyn Error>> {

    let mut rdr = csv::Reader::from_reader(io::stdin());
    // Instead of creating an iterator with the 'records' method, we create
    // an iterator with the 'deserialize' method.

    for result in rdr.deserialize() {
        let record  : Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// Return the first positional argument sent to this process. If there are
// no positional arguments, then this returns an errors.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
