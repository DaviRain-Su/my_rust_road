#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use mini_redis::{client, Result};
use bytes::Bytes;

use tokio::sync::mpsc;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

use log::{info, error};
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String, 
        val: Bytes,
        resp: Responder<()>,
    }
}
#[tokio::main]
pub async fn main() -> Result<()> {


    let (tx, mut rx) = mpsc::channel(23);

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get {key, resp } => {
                    let ret = client.get(&key).await;
                    // info!("😆Get = {:?}", ret);
                    println!("😆Get = {:?}", ret);
                    let _ = resp.send(ret);
                }
                Set { key, val, resp } => {
                    let ret = client.set(&key, val).await;
                    let _ = resp.send(ret);
                }
            }
        }
    });
    let tx0 = tx.clone();
    // let tx2 = tx.clone();
    // let tx3 = tx.clone();

    // set hello -> world
    let t0 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set{
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx,
        };

        if tx0.send(cmd).await.is_err() {
            error!("❎connection task shutdown");
            return;
        }

        let res = resp_rx.await;
        // info!("😄GOT = {:?}", res);
        println!("😆Get = {:?}", res);
    }).await?;

    // get hello 
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get{ 
            key: "hello".to_string(),
            resp: resp_tx,
        };

        if tx.send(cmd).await.is_err() {
            error!("❎connection task shutdown");
            return;
        };
        
        let res = resp_rx.await;
        // info!("😄GOT = {:?}", res);
        println!("😄GOT = {:?}", res);
    }).await?;

    // // set foo -> bar
    // let t2 = tokio::spawn(async move {
    //     let cmd = Command::Set{
    //         key: "foo".to_string(),
    //         val: "bar".into(),
    //     };

    //     tx2.send(cmd).await.unwrap();
    // }).await?;

    // // get foo 
    // let t3 = tokio::spawn(async move {
    //     let cmd = Command::Get{
    //         key: "foo".to_string(),
    //     };

    //     tx3.send(cmd).await.unwrap();
    // }).await?;

    // t1.await.unwrap();
    // t2.await.unwrap();
    // t3.await.unwrap();
    manager.await.unwrap();
    // let tx2 = tx.clone();
    // // open a connection to the mini redis address.
    // // let mut client = client::connect("127.0.0.1:9000").await?;

    // tokio::spawn(async move {
    //     // let _res = client.get("hello").await;
    //     tx.send("send from first handle").await;
    // });

    // tokio::spawn(async move {
    //     // client.set("foo", "bar".into()).await;
    //     tx2.send("send from second handle").await;
    // });


    // while let Some(message) = rx.recv().await {
    //     println!("GOT = {}", message);
    // }
    // set the "hello" with value "world"
    // client.set("1", "3".into()).await?;

    // Get key "hello"
    // let result = client.get("1").await?;

    // println!("got value from the server, result = {:?}", result);

    Ok(())
}
