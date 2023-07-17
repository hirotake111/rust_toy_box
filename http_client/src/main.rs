use std::sync::Arc;
use std::time::Instant;
use std::{env, vec};

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

use http_client::fetch;

// Single HTTP request

#[allow(dead_code)]
#[tokio::main]
async fn main_() -> Result<(), Box<dyn std::error::Error>> {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            // Display a friendly message
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    println!("Making an HTTP request to {url}...");
    let start = Instant::now();
    let response = fetch(&url).await?;
    println!(
        "Received a response from {} - Status code: {}",
        url, response
    );
    println!("(The request took {:?})", start.elapsed());
    Ok(())
}

// Multiple HTTP requests

#[allow(dead_code)]
#[tokio::main]
async fn main___() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec!["https://www.google.com", "https://www.rust-lang.org"];
    let mut join_handles = vec![];

    for url in urls {
        println!("Making an HTTP request to {url}");
        join_handles.push(tokio::spawn(async move {
            match fetch(&url).await {
                Ok(status_code) => {
                    println!("Received a response from {url} - Status code: {status_code}")
                }
                Err(err) => println!("ERROR: {:?}", err),
            }
        }));
    }

    // Ensure that all async tasks have been completed before proceeding
    for handle in join_handles {
        handle.await?;
    }
    Ok(())
}

// Multiple HTTP requests with semaphore

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(2)); // Limit the number of concurrent requests to 3.
    let urls = vec![
        "https://www.google.com",
        "https://www.rust-lang.org",
        "https://www.facebook.com",
        "https://www.linkedin.com",
        "https://www.microsoft.com",
    ];
    let mut join_handles = vec![];

    for url in urls {
        let permit = semaphore.clone().acquire_owned().await?;
        println!("Acquired a permission for {url}");
        join_handles.push(tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;
            if let Ok(code) = fetch(&url.to_string()).await {
                println!("Received a response from {} - Status code: {}", url, code);
            } else {
                println!("Failed to get a response from {url}");
            }
            drop(permit);
        }));
    }

    println!("Queued all handles");
    for handle in join_handles {
        handle.await?;
        println!("await done");
    }
    Ok(())
}
