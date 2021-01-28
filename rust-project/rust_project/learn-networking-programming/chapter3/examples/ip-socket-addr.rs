#![feature(ip)]

use std::net::{IpAddr, SocketAddr};

fn main() {
    //constuct an IpAddr from a string and check it
    // represents the loopback address
    let local: IpAddr = "127.0.0.1".parse().unwrap();
    assert!(local.is_loopback());

    //construct a globally routable IPV6 address from individual
    //octets
    //and assert it is classified correctly
    //
    let global: IpAddr = IpAddr::from([0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1]);
    assert!(global.is_global());

    // construct a SockAddr  from a string an assert that the underlying
    // IP is IPV4 address
    let local_sa: SocketAddr = "127.0.0.1:80".parse().unwrap();
    assert!(local_sa.is_ipv4());

    //constuct a SockAddr from a IPV6 address and a port, assert that
    //the underlying address is indeed IPV6
    let global_sa = SocketAddr::new(global, 80u16);
    assert!(global_sa.is_ipv6());
}
