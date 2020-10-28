use mini_redis::{client};
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;


/// provided by the requester and used by the manager task to send 
/// the command response back to the requester. 
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
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
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    // println!("Got = {}", std::str::from_utf8(&res).unwrap());
                    // Ignore errors 
                    let _ = resp.send(res);
                },
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore errors 
                    let _ = resp.send(res);
                }
            }
        }
    });

    
    // The "Sender" handles are moved into the tasks. As there are two 
    // tasks, we need a second "Sender". 
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key 
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };
        // Send the Get request 
        tx.send(cmd).await.unwrap();

        //Await the response 
        let res = resp_rx.await.unwrap().unwrap().unwrap();
        println!("Got = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "454554".into(),
            resp: resp_tx,
        };

        // Send the SET request 
        tx2.send(cmd).await.unwrap();

        //Await the response 
        let res = resp_rx.await.unwrap().unwrap();
        println!("Got = {:?}", res);
    });
    
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
