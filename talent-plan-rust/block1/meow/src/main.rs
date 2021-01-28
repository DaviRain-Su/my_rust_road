// // main.rs
// // use std::env;
// use clap::Clap;
// #[derive(Clap)]
// #[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
// struct Opts {
    
//     #[clap(short, long, default_value = "default.conf")]
//     config: String,
    
//     input: String,
   
//     #[clap(short, long, parse(from_occurrences))]
//     verbose: i32,
//     #[clap(subcommand)]
//     subcmd: SubCommand,
// }

// #[derive(Clap)]
// enum SubCommand {
//     #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
//     Test(Test),
// }
// #[derive(Clap)]
// struct Test {
//     #[clap(short)]
//     debug: bool
// }

// fn main() {
//     let opts: Opts = Opts::parse();

//     println!("Value for config: {}", opts.config);
//     println!("Using input file: {}", opts.input);

//     match opts.verbose {
//         0 => println!("No verbose info"),
//         1 => println!("Some verbose info"),
//         2 => println!("Tons of verbose info"),
//         3 | _ => println!("Don't be crazy"),
//     }

//     match opts.subcmd {
//         SubCommand::Test(t) => {
//             if t.debug {
//                 println!("Printing debug info...");
//             } else {
//                 println!("Printing normally...");
//             }
//         }
//     }
// }
// // fn main() {
// //     let args :Vec<String> = env::args().collect();
// //     println!("{:?}", args);
// //     // process::exit(1);
// // }

use dotenv_codegen::dotenv;
use std::env;
fn main() {
    println!("port = {}", dotenv!("PORT"));
    println!("path = {}", dotenv!("PATH"));
    println!("mode = {}", dotenv!("MODE"));
    println!("zone = {}", dotenv!("ZONE"));
    println!("area = {}", dotenv!("AREA"));
    

    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key)
    }
    
}