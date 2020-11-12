// This makes the csv crate accessible to your program.
extern crate csv;
extern crate serde;

// This lets us write '#[derive(Derserialize)]'
#[macro_use]
extern crate serde_derive;


// Import the standard library's I/O module so we can read from stdin.
use std::error::Error;
use std::io;
use std::ffi::OsString;
use std::process;
use std::env;

// We don't need to derive 'Debug' (which doesn't require Serde), but it's a good
// habit to do it for all your types.
//
// Notice that the field names in this struct are NOT in the same order as the
// fields in the CSV data!
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")]
struct Record {
    #[serde(rename = "Latitude")]
    latitude : f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
    #[serde(rename = "Population")]
    population: Option<u64>,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "State")]
    state: String,
}


// The 'main' function is where your program starts executing.
fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}


fn run() -> Result<(), Box<dyn Error>> {

    let mut rdr = csv::Reader::from_reader(io::stdin());
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
