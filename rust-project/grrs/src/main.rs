use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use anyhow::{Context, Result};
// use indicatif;
use log::{info, warn, trace};

use grrs::pattern;

/// Search for a pattern in a file and display the line that contain it. 
#[derive(Debug, StructOpt)]
struct Opt {
    
    /// The pattern to look for
    // #[structopt(default_value ="main")] 
    #[structopt(name="PATTERN")]
    pattern : String, // 需要查找的字符串
    
    /// The path to the file to read
    // #[structopt(parse(from_os_str), default_value = "src/main.rs")]
    #[structopt(name = "PATH",parse(from_os_str))]
    path : std::path::PathBuf, // 需要查找的文件路径  
}

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl From<io::Error> for CustomError {
//     fn from(err : io::Error) -> Self {
//         Self(err.to_string())
//     }
// }

// impl From<()> for CustomError{
//     fn from( _t : ()) -> Self {
//         Self("Ok".to_string())
//     }
// }
// struct CliResult(Result<(), CustomError>);
// type CliResult = Result<(), CustomError>;
// impl std::fmt::Display  for CliResult{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0.err().unwrap())
//     }
// }
// fn main() -> Result<()> {
// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<()>{
    env_logger::init();
    trace!("start Cli");
    info!("starting up!");
    warn!("oops, nothing implemented!");


    let args = Opt::from_args();
    trace!("{:?}", args);
    // println!("{:#?}", args);

    // let f = File::open(&args.path)
    //     .map_err(|err| {
    //         CustomError(format!("Error reading : `{:?}` : {}", &args.path, err)); 
    //     })?;

    let f = File::open(&args.path)
        .with_context(|| format!("Could not read file `{:?}`", &args.path))?;
    
    trace!("{:?}", f);
    // let f = File::open(&args.path)?;
    // let f = File::open(&args.path)
    //     .map_err(|err| CustomError(format!("Error reading `{:?}` : {}", &args.path, err)))?;

    // let mut reader = BufReader::new(f);
    let mut reader = BufReader::with_capacity(10 , f);
    
    trace!("{:?}", reader);

    let mut content = String::new();
    
    // version 1
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");
    
    // version 2
    // let len = reader.read_to_string(&mut content)
    //     .map_err(|err| {
    //         CustomError(format!("Error reading : `{:?}` : {}", &args.path, err));
    //     })?;

    let len = reader.read_to_string(&mut content)
        .with_context(|| format!("Could not read file `{:?}`", &args.path))?;

    trace!("{}",len);
    
    // let result = reader.read_to_string(&mut content)
    //     .map_err(|error| CustomError(format!("Error Can't read content : {}", error)))?;
    
    // let len = match result {
    //     Ok(len) => {
    //         println!("read len is : {}", len);
    //         len
    //     },
    //     Err(error) => { 
    //         // panic!("Oh noes: {}", error);
    //         return Err(error.into())
    //     },
    // };

    // println!("len : {}", len);
    // println!("len : {}", result);
    // println!("line =s {}", line); // ok 
    

    // 进度条
    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
        
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    trace!("{:?}", handle);
    
    // line_num index from 0
    // for (line_num,line) in content.lines().enumerate() {
    //     if line.contains(&args.pattern) {
    //         // println!("{}", line);
    //         // so this line_num must plus one, index from 1
    //         writeln!(handle, "{:4}: {}",line_num + 1, line)?;
    //         trace!("{}: {}", line_num + 1, line);
    //         // handle.flush()?;
    //     }
    // }  
    pattern::find_matches(content.as_str(), &args.pattern, &mut handle);  


    Ok(())
}


