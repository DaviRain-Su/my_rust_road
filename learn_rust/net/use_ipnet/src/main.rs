use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let v4 = Ipv4Net::new(Ipv4Addr::new(127, 0, 0, 1), 24).unwrap();
    let v6 = Ipv6Net::new(Ipv6Addr::new(0, 0, 0, 0,0,0,0,1), 24).unwrap();
    println!("v4 = {:?}", v4);
    println!("v6 = {:?}", v6);
    
    let v4 = Ipv4Net::from_str("10.1.1.0/24").unwrap();
    let v6 = Ipv6Net::from_str("fd00::/24").unwrap();
    println!("v4 = {}", v4);
    println!("v6 = {}", v6);

    let v4 : Ipv4Net = "10.1.1.0/24".parse().unwrap();
    let v6 : Ipv6Net = "fd00::/24".parse().unwrap();
    println!("v4 = {}", v4);
    println!("v6 = {}", v6);

    let net =  IpNet::from(v4);
    println!("net = {}", net);

    let net = IpNet::from_str("127.0.0.0/24").unwrap();
    println!("net = {}", net);

    let net : IpNet = "127.0.0.1/24".parse().unwrap();
    println!("net = {}", net);

    println!("{}, hostmask = {}", net, net.hostmask());
    println!("{}, netmask = {}", net, net.netmask());

    assert_eq!(
        "192.168.12.34/16".parse::<IpNet>().unwrap().trunc(),
        "192.168.0.0/16".parse::<IpNet>().unwrap()
    );

    Ok(())
}
