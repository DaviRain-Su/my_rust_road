use std::process::exit;
use structopt::StructOpt;

/// Set the value of a string key to a string
#[derive(Debug, StructOpt)]
pub struct Set {
    /// A string key
    #[structopt(name = "KEY")]
    pub key: Option<String>,
    /// The string value of the key
    #[structopt(name = "VALUE")]
    pub value: Option<String>,
}

/// Get the string value of a given string key
#[derive(Debug, StructOpt)]
pub struct Get {
    /// A string key
    #[structopt(name = "KEY")]
    pub key: Option<String>,
}

/// Remove a given key
#[derive(Debug, StructOpt)]
pub struct Rm {
    /// A string key
    #[structopt(name = "KEY")]
    pub key: Option<String>,
}
#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "set")]
    Set(Set),
    #[structopt(name = "get")]
    Get(Get),
    #[structopt(name = "rm")]
    Rm(Rm),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "kvs")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(subcommand)]
    // #[structopt(flatten)]
    pub command: Command,
}

fn main() {
    let opt = Opt::from_args();

    // println!("{:#?}", opt);

    match opt.command {
        Command::Set(_set) => {
            eprint!("unimplemented");
            exit(1)
        }
        Command::Get(_get) => {
            eprint!("unimplemented");
            exit(1)
        }
        Command::Rm(_rm) => {
            eprint!("unimplemented");
            exit(1)
        }
    }
}
