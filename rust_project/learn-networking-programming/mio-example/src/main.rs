use std::error::Error;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

//Some tokens to allow us to identify which evnent is for which socket.
const SERVER: Token = Token(0);
//const CLIENT: Token = Token(1);

fn main() -> Result<(), Box<dyn Error>> {
    //Create a poll instance
    let mut poll = Poll::new()?;

    //create storage for events.
    let mut events = Events::with_capacity(128);

    //Setup the server socket.
    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;
    //start listening for incoming connection.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    //Set the client socket.
    //let mut client = TcpStream::connect(addr)?;
    //Register the socket
    //poll.registry()
    //    .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    //start an event loop
    loop {
        // Poll Mio for events, blocking util we get an event.
        poll.poll(&mut events, None)?;

        //Process each event
        for event in events.iter() {
            // We can use the token we previousy provided to 'register' to
            // determine for which socket the event it
            match event.token() {
                SERVER => {
                    //if this is an event for the server, it means a connection
                    //is ready to be accepted.

                    //Accept the connection and drop it immediately. This will
                    //close  the socket and notify client of the EOF
                    let connection = server.accept();
                    println!("Connection from {:?}", connection);
                }
                /*
                CLIENT => {
                    if event.is_readable() {
                        // we can (likely) write to the socket without blocking
                    }
                    if event.is_writable() {
                        // we can likely read from the socket without blocking
                    }

                    //Since the server just shuts down the connection, let's
                    //just exit from our event loop.
                    return Ok(());
                }
                */
                // We don't expect any events with tokens other than those we provide.
                _ => unreachable!(),
            }
        }
    }
}
