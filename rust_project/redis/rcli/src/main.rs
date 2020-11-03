use bytes::{BufMut, BytesMut};
use std::error::Error;
use tokio::net::TcpStream;
use tokio::prelude::*;

use structopt::StructOpt;
mod commands;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    // let com = commands::Commands::from_args();
    // println!("com = {:?}", com);
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut buf = [0u8; 1024];
    let mut resp = BytesMut::with_capacity(1024);

    let (mut reader, mut writer) = stream.split();

    let comm = commands::Commands::from_args();

    // 向服务器发送ping
    // writer.write(b"*1\r\n$4\r\nPING\r\n").await?;
    writer.write(&comm.to_bytes()).await?;
    let n = reader.read(&mut buf).await?;

    // 将buf放入到resp
    resp.put(&buf[0..n]);
    // 返回结果 应该是PONG
    // println!("{:?}", resp);
    println!("{}", std::str::from_utf8(&buf[..n]).unwrap());
    Ok(())
}
