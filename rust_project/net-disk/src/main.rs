#[macro_use]
extern crate diesel;
extern crate dotenv;

use log::{debug, error, info, log_enabled, Level};
use serde::{Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;
use std::io::{self, prelude::*, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

mod cli;
mod command;
mod db;
mod logic;
mod threadpool;
mod users;
mod utils;

use logic::{recv_message, send_message};

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    debug!("Incoming connection from : {}", stream.peer_addr().unwrap());

    let mut buf = Vec::new();
    let mut stream = BufReader::new(stream);

    loop {
        buf.clear();

        // let bytes_read = stream.read_until(b'\n', &mut buf)?;
        // if bytes_read == 0 {
        //     return Ok(());
        // }

        // 先读取注册信息
        recv_message(&mut stream, &mut buf);
        debug!("buf = {:?}", buf);

        // 根据接受的是login / register / cancel 去选择执行
        let lrc_command: logic::LRC = serde_json::from_slice(&buf)?;
        debug!("lrc_command = {:?}", lrc_command);

        // 大部分的处理任务都放在了登录之后的操作，
        // 例如，登录成功之后需要查看网盘的内容，ls,
        // 目录的切换操作 cd
        // 将本地文件发送到服务器 puts
        // 下载服务器上的文件 gets
        // 删除服务器上的文件 rm, 删除的只是每个用户的虚拟文件系统中的文件名，当真实的文件引用数变为0，删除真实的文件
        // 其他命令不响应
        // 登录注册取消的处理逻辑
        lrc_command.deal_lrc(&mut stream);

        // let input: command::Commands = serde_json::from_slice(&buf)?;
        // debug!("input = {:?}", input);

        // stream
        //     .get_mut()
        //     .write_all(serde_json::to_string(&input).unwrap().as_bytes())
        //     .expect("Failed to write to server");
        // stream
        //     .get_mut()
        //     .write_all(b"\n")
        //     .expect("Failed to write to server");
        // stream.get_mut().flush()?;
    }
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = cli::Config::new("./Cargo.toml");

    // Use RUST_LOG=debug cargo run
    debug!("config = {:?}", config);

    let ip = config.get_ip();
    let port = config.get_port();
    let thread_num = config.get_thread_num();
    debug!("ip = {}", ip);
    debug!("port = {}", port);
    debug!("thread_num = {}", thread_num);

    let ip_port = config.get_ip_port();
    debug!("ip_port = {}", ip_port);

    let listener = TcpListener::bind(ip_port)?;
    // listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let pool = threadpool::ThreadPool::new(thread_num);

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Use threadpool
                pool.execute(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                eprintln!("failed: {}", e);
            }
        }
    }
    Ok(())
}
