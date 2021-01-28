use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        
        let db = db.clone();
        // A new task is spawned for each inbound socket. The socket is 
        // moved to the new task and processed there.
        println!("Accepted"); 
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
   use mini_redis::Command::{self, Get, Set};
   
   // Connection, provided by 'mini-redis', 
   // handles parsing frames the socket
   let mut connection = Connection::new(socket);

   // Use 'read_frame' to receive a command from the connection. 
   while let Some(frame) = connection.read_frame().await.unwrap() {
       let response = match Command::from_frame(frame).unwrap() {
           Set(cmd) => {
               let mut db = db.lock().unwrap();
               // The  value is stored as 'Vec<u8>' 
               db.insert(cmd.key().to_string(), cmd.value().clone());
               Frame::Simple("OK".to_string())
           }
           Get(cmd) => {
               // 'Frame::Bulk' expects data to be of type 'Bytes'. 
               // This type will be covered later in the tutorial. 
               // For now, '&Vec<u8>' is converted to 'Bytes' using 'into()'
               let db = db.lock().unwrap();
               if let Some(value) = db.get(cmd.key()) {
                   Frame::Bulk(value.clone().into())
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