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
    let (mut tx, mut rx) = mpsc::channel(32);

    // Establish a connection to the server  
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    {   
        //Spawn two tasks, one gets a key, the other sets a key
        let t1 = tokio::spawn(async {
            client.get("hello").await;
        });
        t1.await.unwrap();
    }

    // {
    //     let t2 = tokio::spawn(async {
    //         client.set("foo", "bar".into()).await;
    //     });
    //     t2.await.unwrap();
    // }
}
