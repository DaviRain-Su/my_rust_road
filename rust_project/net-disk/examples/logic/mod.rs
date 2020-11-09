use std::io::BufReader;
use std::io::{self, Error};
use std::net::TcpStream;

pub fn get_intput(buf: &mut String) {
    io::stdin().read_line(buf).expect("Failed read from stdin");
}

pub fn login(stream: &mut BufReader<TcpStream>) -> Result<(), Error> {
    let mut username = String::new();
    let mut password = String::new();

    get_intput(&mut username);
    get_intput(&mut password);

    unimplemented!()
}
