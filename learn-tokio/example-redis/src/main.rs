// #![allow(unused_must_use)]

use log::info;

use mini_redis::{Connection, Frame, Result};

use mini_redis::Command::{self, Get, Set};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init(); // must need

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        info!("🎈socket: {:?}", socket);
        let handle = tokio::spawn(async move {
            info!("😁Entry tokio spawn");
            process(socket).await
        });

        let _ret = handle.await?;
    }
}

async fn process(socket: TcpStream) -> Result<()> {
    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);
    info!("🌲connection : {:?}", connection);

    while let Some(frame) = connection.read_frame().await? {
        info!("📚frame : {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                info!("👌Entry Set!");
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                info!(
                    "K: {:?}, V: {:?}",
                    cmd.key().to_string(),
                    cmd.value().to_vec()
                );
                Frame::Simple("Ok".to_string())
            }
            Get(cmd) => {
                info!("👌Entry Get!");
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => Frame::Error(format!("unimplemented {:?}", cmd)),
        };

        connection.write_frame(&response).await?;
    }

    Ok(())
}
