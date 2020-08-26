use structopt::StructOpt;
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt.subcommand);
}


/// A utility for interaction with nuget packages
#[derive(StructOpt, Debug)]
#[structopt(name = "nuget")]
struct Opt {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    Install(Install),
}


#[derive(Debug, StructOpt)]
pub struct Install {}