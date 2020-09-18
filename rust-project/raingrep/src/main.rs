use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use log::{info, warn, trace};

use raingrep::pattern;

/// Search for a pattern in a file and display the line that contain it. 
#[derive(Debug, StructOpt)]
struct Opt {
    
    /// The pattern to look for
    #[structopt(name="PATTERN")]
    pattern : String, // 需要查找的字符串
    
    /// The path to the file to read
    #[structopt(name = "PATH",parse(from_os_str))]
    path : std::path::PathBuf, // 需要查找的文件路径  
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    env_logger::init();
    trace!("start raingrep");

    let args = Opt::from_args();
    trace!("{:?}", args);
    
    let f = File::open(&args.path)
        .map_err(|err| {
            CustomError(format!("Error reading `{:?}` : {}", &args.path, err))
        })?;
    trace!("{:?}", f);

    let mut reader = BufReader::with_capacity(10 , f);
    trace!("{:?}", reader);

    let mut content = String::new();
    
    
    let len = reader.read_to_string(&mut content)
        .map_err(|err|  { 
            CustomError(format!("Error reading : `{:?}` : {}", &args.path, err))
        })?;
    trace!("{}",len);
    
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    trace!("{:?}", handle);
    
    pattern::find_matches(content.as_str(), &args.pattern, &mut handle);  

    Ok(())
}


