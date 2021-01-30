#![allow(dead_code)]
use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection to the mini redis address.
    let mut client = client::connect("127.0.0.1:9000").await?;

    // let t1 = tokio::spawn(async {
        // let _res = client.get("hello").await;
    // });

    // let t2 = tokio::spawn(async {
        // client.set("foo", "bar".into()).await;
    // });

    // set the "hello" with value "world"
    client.set("1", "3".into()).await?;

    // Get key "hello"
    let result = client.get("1").await?;

    println!("got value from the server, result = {:?}", result);

    // t1.await.unwrap();
    // t2.await.unwrap();
    Ok(())
}
