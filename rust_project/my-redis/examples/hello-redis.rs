use mini_redis::{client};
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String, 
        val: Bytes,
    },
}
#[tokio::main]
pub async fn main(){

    // Create a new channel with a capacity of at most 32. 
    let (tx, mut rx) = mpsc::channel(32);

    // The 'move' keyword is usedd to **move** ownership of 'rx' into task 
    let manager = tokio::spawn(async move {
        // Esatblish a connection to the server 
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages 
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            
            match cmd {
                Get { key } => {
                    let res = client.get(&key).await.unwrap().unwrap();
                    println!("Got = {}", std::str::from_utf8(&res).unwrap());
                },
                Set { key, val} => {
                    client.set(&key, val).await.unwrap();
                }
            }
        }
    });


    // The "Sender" handles are moved into the tasks. As there are two 
    // tasks, we need a second "Sender". 
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key 
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string(),
        };
        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "454554".into(),
        };
        tx2.send(cmd).await.unwrap();
    });
    
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
