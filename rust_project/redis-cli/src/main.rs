use async_std::io;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    let mut client = Client::new("localhost:6379").await?;
    println!(
        "{:?}",
        client.set("davirain".into(), "shanghai".into()).await
    );
    println!("{:?}", client.get("davirain".into()).await.unwrap());
    println!("{:?}", client.ping().await.unwrap());
    Ok(())
}

fn parse_response(buffer: &[u8]) -> Result<&str, Error> {
    if buffer.is_empty() {
        return Err(Error {
            estr: "Empty buffer".into(),
        });
    }

    if buffer[0] == b'-' {
        return Err(Error {
            estr: format!("Error Response: {:?}", &buffer[1..buffer.len() - 2]),
        });
    }

    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
}

struct Client {
    stream: TcpStream,
}

impl Client {
    async fn new<A: ToSocketAddrs>(addr: A) -> Result<Client, io::Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Client { stream })
    }
}

impl Client {
    async fn set(&mut self, key: String, value: String) -> Result<(), Error> {
        let command = RespValue::Array(vec![
            RespValue::BlukString(b"SET".to_vec()),
            RespValue::BlukString(key.into_bytes()),
            RespValue::BlukString(value.into_bytes()),
        ]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        parse_response(&buffer[..bytes_read])?;
        Ok(())
    }
    async fn get(&mut self, key: String) -> Result<String, Error> {
        let command = RespValue::Array(vec![
            RespValue::BlukString(b"GET".to_vec()),
            RespValue::BlukString(key.into_bytes()),
        ]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        let resp = parse_response(&buffer[..bytes_read])?;
        Ok(resp.to_owned())
    }

    async fn ping(&mut self) -> Result<String, Error> {
        let command = RespValue::Array(vec![RespValue::BlukString(b"PING".to_vec())]);
        let mut buffer = vec![];
        command.serialize(&mut buffer);
        self.stream.write_all(&buffer).await?;

        let bytes_read = self.stream.read(&mut buffer).await?;
        let resp = parse_response(&buffer[..bytes_read])?;
        Ok(resp.to_owned())
    }
}

#[derive(Debug)]
struct Error {
    estr: String,
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            estr: e.to_string(),
        }
    }
}
enum RespValue {
    SimpleString(String),
    Error(Vec<u8>),
    Integer(i64),
    BlukString(Vec<u8>),
    Array(Vec<RespValue>),
}

impl RespValue {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            RespValue::Array(values) => {
                buf.push('*' as u8);
                buf.append(&mut format!("{}", values.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                for value in values {
                    value.serialize(buf);
                }
            }
            RespValue::BlukString(mut data) => {
                buf.push(b'$');
                buf.append(&mut format!("{}", data.len()).into_bytes());
                buf.push('\r' as u8);
                buf.push('\n' as u8);
                buf.append(&mut data);
                buf.push('\r' as u8);
                buf.push('\n' as u8);
            }
            _ => unimplemented!(),
        }
    }
}
