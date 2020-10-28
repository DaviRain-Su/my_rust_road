use std::rc::Rc;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::task;
use tokio::task::yield_now;

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Calling 'say_world()' does not execute the body of "say_world"
    let op = say_world();

    // This println! comes first
    println!("Hello, world!");

    // Calling '.await?' on 'Op' starts executing 'say_world'
    op.await;

    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Her'e vec is {:?}", v);
    })
    .await
    .unwrap();

    tokio::spawn(async {
        // The scope forces 'rc' to drop before '.await'
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // 'rc' is no longer used. It is **not** persisted when
        // the task yeilds to the scheduler
        yield_now().await;
    })
    .await
    .unwrap();

    // Tasks are Send when all data that is held across .await calls is Send.
    /*
     * tokio::spawn(async {
     *    let rc = Rc::new("hello");
     *        // has type std::rc::Rc<&str> which is not 'Send'
     *   yield_now().await;
     *        // await occurs here, with 'rc' maybe used later
     *
     *    println!("{}", rc);
     *}).await.unwrap(); // 'rc' is later dropped here
     */
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }

    // async read()
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 1024];

    //read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!(
        "the message: {}",
        std::str::from_utf8(&buffer[..n]).unwrap()
    );

    //async read_to_end()
    let mut f = File::open("src/main.rs").await?;
    let mut buffer = Vec::new();

    //read the whole file
    let s = f.read_to_end(&mut buffer).await?;
    println!(
        "the message:\n{}",
        std::str::from_utf8(&buffer[..s]).unwrap()
    );
    // async write_all()
    let mut buffer = File::create("bar.txt").await?;

    buffer.write_all(b"some bytes").await?;

    // async copy()
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
