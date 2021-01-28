use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame, Result};


#[tokio::main]
async fn main() -> Result<()>{
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process(socket).await?;
    }
}

async fn process(socket: TcpStream) -> Result<()>{
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await? {
        println!("Got: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await?;
    }

    Ok(())
}