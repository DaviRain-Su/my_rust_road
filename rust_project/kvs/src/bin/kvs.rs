use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs")]
/// A key-value database
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum  Command {
    #[structopt(name = "set")]
    /// Set the value of a string key to a string
    Set {
        key: String,
        value: String,
    },
    #[structopt(name = "get")]
    /// Get the string value of a given string key
    Get {
        key: String,
    },
    #[structopt(name = "rm")]
    /// Remove a given key
    Rm {
        key: String,
    }
}
fn main() {
    let opts : Opts = Opts::from_args();

    match opts.cmd {
        Command::Set {
            key: _key,
            value: _value,
        } => {
            eprintln!("unimplemented");
            exit(1);
        },
        Command::Get {
            key: _key,
        } => {
            eprintln!("unimplemented");
            exit(1);
        },
        Command::Rm {
            key: _key,
        } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
