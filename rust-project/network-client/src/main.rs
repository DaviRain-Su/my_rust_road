use std::net::UdpSocket;
use std::net::IpAddr;
use std::net::Ipv4Addr;
fn main() {
    const DOMAIN_NAME_REQUEST: &[u8] = b"Alice";
    const DNS_SERVER: &str = "1.2.3.4:53";

    let  socket = UdpSocket::bind("127.0.0.1:89").expect("Couldn't bind to address");

    socket.send_to(DOMAIN_NAME_REQUEST, DNS_SERVER)
        .expect("Couldn't send message");
    
    let mut response_data_buffer = [0u8; 4];
    let (size, sender) = socket
        .recv_from(&mut response_data_buffer)
        .expect("failed to receiver data");
    
    println!("size = {}, sender = {}", size, sender);

    let ip_address = IpAddr::V4(Ipv4Addr::from(response_data_buffer));
    println!("ip address = {}", ip_address);
    // println!("Hello, world!");
}
