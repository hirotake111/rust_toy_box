use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[allow(dead_code)]
type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {addr}");
    // let db: Db = Arc::new(Mutex::new(HashMap::new()));
    let db = new_sharded_db(5);

    loop {
        let (socket, _) = listener.accept().await?;
        // Clone the handle to the hash map
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            let _ = process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: ShardedDb) -> Result<()> {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await? {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Set(cmd) => {
                let sharded_key = calculate_hash(&cmd.key()) % db.len();
                println!("sharded_key: {sharded_key}");
                let shard = &db[sharded_key];
                match shard.lock() {
                    Ok(mut shard) => {
                        shard.insert(cmd.key().to_string(), cmd.value().clone());
                        Frame::Simple("OK".to_string())
                    }
                    Err(err) => {
                        Frame::Error(format!("Failed obtaining lock: {:?}", err).to_string())
                    }
                }
            }
            Get(cmd) => {
                let sharded_key = calculate_hash(&cmd.key()) % db.len();
                println!("sharded_key: {sharded_key}");
                let shard = &db[sharded_key];
                match shard.lock() {
                    Ok(shard) => {
                        if let Some(value) = shard.get(cmd.key()) {
                            // `Frame::Bulk` expects data to be of type `Bytes`.
                            // This type will be covered later in the tutorial.
                            // For now, `&Vec<u8>` is converted to `Bytes` using `into()`.
                            Frame::Bulk(value.clone().into())
                        } else {
                            Frame::Null
                        }
                    }
                    Err(err) => {
                        Frame::Error(format!("Failed obtaining lock: {:?}", err).to_string())
                    }
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await?;
    }
    Ok(())
}

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

fn calculate_hash<T: Hash>(t: &T) -> usize {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish() as usize
}
