use mio::net::TcpListener;
// use mio::event::Event;
use mio::{Events, Interest, Poll, Token};

use std::io;

const SERVER: Token = Token(0);



fn main() ->  io::Result<()> {

    let mut poll = Poll::new()?;

    let mut events  = Events::with_capacity(128);

    let addr  = "127.0.0.1:9000".parse().unwrap();

    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;
    
    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    match server.accept() {
                        Ok((_connection, address)) => {
                            println!("Got a connection from : {}", address);
                        }
                        Err(ref err) if would_block(err) => break,
                        Err(err) => return Err(err),
                    }
                },
                _ => unimplemented!(),
            }
        }
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}