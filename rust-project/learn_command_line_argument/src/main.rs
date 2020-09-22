use std::env;
// use std::ffi::OsString;
use std::io;
use std::fs;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text: \n{}", contents);

    // print the command line argument use args()
    // for argument in env::args() {
    //     println!("{}", argument);
    // }

    // let pattern = env::args().nth(1).expect("no given pattern");
    // let path = env::args().nth(2).expect("no given path");
    // println!("pattern = {}, path = {}", pattern, path);


    // print the commanf line arguments use args_os()
    // for argument in env::args_os() {
    //     println!("{:?}", argument);
    // }

    // print the curretn dir
    // let path = env::current_dir()
    //     .map_or(OsString::from(""), |path| {
    //         path.into_os_string()
    //     }); 

    // println!("path = {}", path.into_string().ok().unwrap());

    // print the current exe
    // let current_exe = env::current_exe();
    // println!("current exe = {:?}", current_exe);

    // print the home dir 
    // let home_path = env::home_dir();
    // println!("home dir = {:?}", home_path);

    // vars return enverments (key, value)
    // for (key, value) in env::vars() {
    //     println!("{} : {}", key, value);
    // }

    Ok(())
}
