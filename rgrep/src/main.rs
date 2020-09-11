use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern : String,
    #[structopt(parse(from_os_str))]
    path : std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    // let pattern = env::args().nth(1).expect("not pattern given");
    // let path = env::args().nth(2).expect("not path given");
    // println!("pattern = {}", pattern);
    // println!("path = {}", path);
    // // println!("Hello, world!");
    // let args = Cli{
    //     pattern : pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    // println!("{:#?}", args);
}
