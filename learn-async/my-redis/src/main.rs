use mini_redis::{client, Result};

#[tokio::main]
pub async fn main()  -> Result<()> {
    
    
    // let mut client = client::connect("127.0.0.1:6379").await?;

    // client.set("hello", "world".into()).await?;

    // let result  = client.get("hello").await?;

    // println!("got value from the server; result = {:?}", result);

    // Ok(())
    let op = hello();

    println!("hello world");

    op.await;

    Ok(())
}

async fn hello() {
    println!("hello davirian");
}
