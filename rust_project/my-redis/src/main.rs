use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
   use mini_redis::Command::{self, Get, Set};
   use std::collections::HashMap;

   // A hashmap is used to store data 
   let mut db = HashMap::new();

   // Connection, provided by 'mini-redis', 
   // handles parsing frames the socket
   let mut connection = Connection::new(socket);

   // Use 'read_frame' to receive a command from the connection. 
   while let Some(frame) = connection.read_frame().await.unwrap() {
       let response = match Command::from_frame(frame).unwrap() {
           Set(cmd) => {
               db.insert(cmd.key().to_string(), cmd.value().clone());
               Frame::Simple("OK".to_string())
           }
           Get(cmd) => {
               if let Some(value) = db.get(cmd.key()) {
                   Frame::Bulk(value.clone())
               }else {
                   Frame::Null
               }
           }, 
           cmd => panic!("unimplemented {:?}", cmd),
       };

       // Write the respinse to the client 
       connection.write_frame(&response).await.unwrap();
   }
}