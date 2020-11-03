use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::net::{TcpListener, TcpStream};
use std::io::{self, Error, Read, Write};
use std::thread;

use log::{ debug, error, log_enabled, info, Level};
mod threadloop;
mod cli;


fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    debug!("Incomming connection from : {}", stream.peer_addr().unwrap());

    let mut buf  = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
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
