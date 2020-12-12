mod ch1;
// use std::io;
// use std::io::Write;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// #[warn(deprecated)]
fn main() {


    // 标准输入输出的
    // print!("请输入一个字符串: "); // 不会出现预期的提示字符串， 因为行没有被刷新。
    // // 如果想要达到预期的效果就要显式的刷新。
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Read Failed!");
    // println!("你输入的字符串是: 12{}", input.trim());

    // 文件的输入输出
    // 创建一个文件路径
    // let path = Path::new("hello.txt");
    // let display = path.display();
    //
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
    //     Ok(file) => file,
    // };
    //
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
    //     Ok(_) => println!("{} contains: \n{}", display, s),
    // }

    static LOREM_IPSUM: &'static str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit,\
        sed do eiusmodtempor incididunt ut labore et dolore magna aliqua.\
        Ut enim ad minim veniam,quis nostrud exercitation ullamco laboris nisi ut aliquip ex eacommodoconsequat.\
        Duis aute irure dolor in reprehenderit in voluptate velit essecillum dolore eu fugiat nulla pariatur.\
        Excepteur sint occaecatcupidatat nonproident,\
        sunt in culpa qui officia deserunt mollit anim id estlaborum.";

    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("Couldn't write to {}: {}", display, Error::description(&why));
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
