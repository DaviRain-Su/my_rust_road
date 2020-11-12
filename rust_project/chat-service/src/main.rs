use mio::{Poll, Interest, Events, Token};
use mio::net::TcpListener;
use std::net::{SocketAddr, TcpListener};



struct WebSocketServer;

// impl Handle for WebSocketServer {
//     type Timeout = usize;
//     type Message = ();
// }


fn main() {



    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();

    let mut listener = TcpListener::bind(address)?;

    let poll : Poll = Poll::new()?;

    let events = Events::with_capacity(128);

    const SERVER: Token = Token(0);

    poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

    
    println!("Hello, world!");

}
