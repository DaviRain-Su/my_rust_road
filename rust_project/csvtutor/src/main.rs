// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::io;
use std::error::Error;
use std::process;
use std::ffi::OsString;
// use std::fs::File;
use std::env;

// The 'main' function is where your program starts executing.
fn main() {
     if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    // let mut rdr =   csv::ReaderBuilder::new()
    //     .has_headers(false)
    //     .from_reader(io::stdin());
    //
    // {
    //     // We nest this call in its own scope because of lifetimes.
    //     let headers = rdr.headers()?;
    //     println!("{:?}", headers);
    // }
    // let headers = rdr.headers()?.clone();
    // println!("{:?}", headers);
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    // We can ask for the headers at any time. There's no need to nest this
    // call in its own scope because we never try to borrow the reader again.
    // let headers = rdr.headers()?;
    // println!("{:?}", headers);
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