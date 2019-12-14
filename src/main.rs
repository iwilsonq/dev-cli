use std::env;
use tokio::io::{self, AsyncWriteExt as _};

use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    let url = url.parse::<hyper::Uri>().unwrap();

    fetch_url(url).await
}
