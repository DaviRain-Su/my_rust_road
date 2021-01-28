use std::net::UdpSocket;
use std::{io, str};

fn main() -> io::Result<()>{
    // println!("Hello, world!");

    let socket = UdpSocket::bind("127.0.0.1:8000")?;

    socket.connect("127.0.0.1:8080")?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;

        let mut buff = [0u8; 1500];
        socket.recv_from(&mut buff)?;
        println!("recv : {}", str::from_utf8(&buff)
            .expect("could not write buffer as string")
        );
    }
    // Ok(())
}
