use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::thread;
use std::time::Duration;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> std::io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    // 创建一个tcp服务端
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    // 注册服务端
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // 创建一个tcp客户端
    let mut client = TcpStream::connect(addr)?;

    // 注册客户端
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let connection = server.accept();
                    println!("SERVER recv a connection!");
                    thread::sleep(Duration::from_secs(1));
                    drop(connection);
                },

                CLIENT => {
                    if event.is_writable() {
                        println!("Client write");
                    }

                    if event.is_readable() {
                        println!("Client read");
                    }

                    return Ok(());
                },

                _ => {
                    unreachable!();
                }
            }
        }
    }

}
