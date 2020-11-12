// This makes the csv crate accessible to your program.
extern crate csv;

// Import the standard library's I/O module so we can read from stdin.
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process;
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
    // let mut rdr = csv::ReaderBuilder::new().has_headers(false)
    //     .delimiter(b';')
    //     .double_quote(false)
    //     .escape(Some(b'\\'))
    //     .flexible(true)
    //     .comment(Some(b'#'))
    //     .from_reader(io::stdin());
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;

        let city = &record[0];
        let state = &record[1];

        // Some records are missing population counts , so if we can't
        // parse a number, treat the population count as missing instead
        // of returning an error.

        let pop: Option<u64> = record[2].parse().ok();
        // Lucky us!  Latitudes and longitudes are available for every record.
        // Therefore, if one couldn't be parsed, return an error.
        let latitude: f64 = record[3].parse()?;
        let longitude: f64 = record[4].parse()?;

        println!(
            "City : {:?}, state: {:?}, pop : {:?}, latitude: {:?}, longitude : {:?}",
            city, state, pop, latitude, longitude
        );
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
