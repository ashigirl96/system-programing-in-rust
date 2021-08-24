use anyhow;
use hyper::{body::HttpBody as _, Client, Uri};
use std::str::FromStr;
use tokio::io::{self, AsyncWriteExt, BufWriter};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let mut buffer = BufWriter::new(io::stdout());

    let uri = Uri::from_str("http://www.example.com").unwrap();
    let mut res = client.get(uri).await?;
    while let Some(chunk) = res.data().await {
        buffer.write_all(&chunk?).await?;
    }
    buffer.flush().await?;
    Ok(())
}
