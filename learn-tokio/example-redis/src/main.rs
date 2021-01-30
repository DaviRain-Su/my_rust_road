#![allow(unused_imports)]
use bytes::Bytes;
use log::{debug, error, info};
use mini_redis::{Command, Connection, Frame, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:9000").await?;

    println!("🎈Listening!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await?;

        let db = db.clone();

        println!("😁Accepted!");

        let ret = tokio::spawn(async move { process(socket, db).await });

        let _result = ret.await.unwrap();
    }
}

async fn process(socket: TcpStream, db: Db) -> Result<()> {
    info!("🎈async function process!");

    use mini_redis::Command::{Get, Set};
    use std::collections::HashMap;

    let mut connection = Connection::new(socket);
    info!("🎈connection = {:?}", connection);

    while let Some(frame) = connection.read_frame().await? {
        info!("🌲 frame = {:?}", frame);

        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
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
