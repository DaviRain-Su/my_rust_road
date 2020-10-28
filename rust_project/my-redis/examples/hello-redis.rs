use mini_redis::{client, Result};

#[tokio::main]
pub async fn main()  -> Result<()> {
    // Open a connection to the mini-redis address. 
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;
    client.set("davirain", "123".into()).await?;

    // Get key "hello"
    // let result = client.get("hello").await?.unwrap();

    // Converts a slice of bytes to a string slice. 
    // let res = std::str::from_utf8(&result).unwrap();

    // println!("got value from the server, result = {}", res);
    // get hello
    println!("hello -- {:?}", client.get("hello").await?.unwrap());
    // get davirain
    println!("davirain --- {:?}", client.get("davirain").await?.unwrap());
    Ok(())
}
