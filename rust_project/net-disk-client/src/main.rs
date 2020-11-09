use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

use log::{debug, error, info, log_enabled, Level};

mod cli;
mod command;
mod logic;
mod threadpool;
mod users;
mod utils;

use logic::{get_intput, login, recv_message, send_message};

fn main() -> io::Result<()> {
    env_logger::init();

    // 从Cargo.tml 读取配置信息
    let config = cli::Config::new("./Cargo.toml");
    debug!("config = {:?}", config);

    // 获取配合的ip, port, thread_num
    let ip = config.get_ip();
    let port = config.get_port();
    let thread_num = config.get_thread_num();
    debug!("ip = {}", ip);
    debug!("port = {}", port);
    debug!("thread_num = {}", thread_num);

    let ip_port = config.get_ip_port();
    debug!("ip_port = {}", ip_port);

    // tcp 连接建立
    let stream = TcpStream::connect(ip_port).expect("Could not connect to server");

    loop {

        utils::main_info();

        let mut input = String::new();
        get_intput(&mut input);
        
        let lrc = logic::LRC::new(&input);
        let mut stream = BufReader::new(&stream);

        // 发送是注册 还是登陆 还是退出信息
        send_message(&mut stream, &lrc)?;
        let ret = lrc.deal(&mut stream);
        debug!("ret = {:?}", ret);
    }
}
