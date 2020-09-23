use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    loop  {
        let mut buffer = [0u8; 1500];
        let (amt, src) = socket
            .recv_from(&mut buffer)
            .expect("failed recv");
        
            println!("size = {}", amt);

        let buf = &mut buffer[..amt];
        buf.reverse();
        socket.send_to(buf, src)?;
    }

    // Ok(())
}
