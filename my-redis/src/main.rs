use std::collections::HashMap;

use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) -> Result<()> {
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await? {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`.
                    // This type will be covered later in the tutorial.
                    // For now, `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await?;
    }
    println!("done processing.");
    Ok(())
}
