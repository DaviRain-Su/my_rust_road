use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use std::io::{self, prelude::*, Error, Read, Write, BufReader};
use log::{ debug, error, log_enabled, info, Level};
use serde::{ Deserialize, Serialize};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

mod threadloop;
mod cli;

#[derive(Debug, Deserialize, Serialize)]
pub enum Commands {
    CD(Option<Vec<String>>),
    LS(Option<Vec<String>>),
    PUTS(Option<Vec<String>>),
    GETS(Option<Vec<String>>),
    REMOVE(Option<Vec<String>>),
    PWD(Option<String>),
    OTHER(String),
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    debug!("Incomming connection from : {}", stream.peer_addr().unwrap());

    let mut buf  = Vec::new();
    let mut stream = BufReader::new(stream);
    loop {
        buf.clear();

        let bytes_read = stream.read_until(b'\n',&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let input: Commands = serde_json::from_slice(&buf)?;
        debug!("input = {:?}", input);

        // stream.get_mut().write(&buf[..bytes_read])?;
        stream.get_mut().write_all(serde_json::to_string(&input).unwrap().as_bytes())
            .expect("Failed to write to server");
        stream.get_mut().write_all(b"\n").expect("Failed to write to server");
        stream.get_mut().flush()?;
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

    let ip_port = format!("{}:{}", ip, port);
    debug!("ip_port = {}", ip_port);

    let listener = TcpListener::bind(ip_port)?;
    // listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let pool = threadloop::ThreadPool::new(thread_num);
    
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
