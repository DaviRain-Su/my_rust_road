use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

use log::{debug, error, info, log_enabled, Level};

mod cli;
mod threadpool;
mod logic;
mod command;
mod users;
mod utils;

use logic::{send_message, recv_message, get_intput, login};


fn main() -> io::Result<()> {
    env_logger::init();
    let config = cli::Config::new("./Cargo.toml");

    debug!("config = {:?}", config);

    let ip = config.get_ip();
    let port = config.get_port();
    let thread_num = config.get_thread_num();

    debug!("ip = {}", ip);
    debug!("port = {}", port);
    debug!("thread_num = {}", thread_num);

    let ip_port = config.get_ip_port();
    debug!("ip_port = {}", ip_port);
    let mut stream = TcpStream::connect(ip_port).expect("Could not connect to server");

    loop {
        utils::main_info();

        let mut input = String::new();
        get_intput(&mut input);
        let lrc = logic::LRC::new(&input);
        
        
        let mut stream = BufReader::new(&stream);
        let ctr = logic::LRC::REGISTRY;
        send_message(&mut stream, &ctr);

        let ret = login(&mut stream);
        debug!("ret = {:?}", ret);
    }
}
