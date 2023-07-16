use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{
    mpsc::{self, Receiver},
    oneshot,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        responder: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        responder: Responder<()>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    let manager = get_manager_handle(rx).await;
    let task1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            responder: resp_tx,
        };
        // Send GET request
        let _ = tx.send(cmd).await;
        // Then await GET response
        let response = resp_rx.await.unwrap().unwrap();
        response
    });
    let task2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            responder: resp_tx,
        };
        // Send SET request
        let _ = tx2.send(cmd).await;
        // Then await SET response
        let response = resp_rx.await.unwrap().unwrap();
        response
    });

    task1.await?;
    task2.await?;
    manager.await?;
    Ok(())
}

async fn get_manager_handle(mut rx: Receiver<Command>) -> tokio::task::JoinHandle<()> {
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, responder } => {
                    let response = client.get(&key).await;
                    let _ = responder.send(response);
                }
                Set {
                    key,
                    value,
                    responder,
                } => {
                    let response = client.set(&key, value).await;
                    let _ = responder.send(response);
                }
            }
        }
        println!("No commands to be consumed.");
    });

    manager
}
