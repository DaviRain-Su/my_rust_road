use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs")]
/// A key-value database
struct Opts {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
enum  Command {
    Set(Set),
    Get(Get),
    Rm(Rm),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "set")]
/// Set the value of a string key to a string
pub struct Set {
    key: String,
    value: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "get")]
/// Get the string value of a given string key
pub struct Get {
    key: String,
}
#[derive(Debug, StructOpt)]
#[structopt(name = "rm")]
/// Remove a given key
pub struct Rm {
    key: String,
}
fn main() {
    let opts : Opts = Opts::from_args();

    match opts.cmd {
        Command::Set(Set{key:_key, value:_value}) => {
            eprintln!("unimplemented");
            exit(1);
        },
        Command::Get(Get{key:_key}) => {
            eprintln!("unimplemented");
            exit(1);
        },
        Command::Rm(Rm{key: _key}) => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
