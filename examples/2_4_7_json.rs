use anyhow::Result;
use reqwest;
use serde_json::{json, to_writer_pretty};
use std::io::Write;

fn main() -> Result<()> {
    println!("Write with os.Stdout at {}", chrono::Local::now());

    let mut writer = std::io::stdout();
    let value = json!({
        "example": "encoding/json",
        "hello": "world",
    });
    to_writer_pretty(writer, &value)?;

    let mut request = reqwest::Request::new(
        reqwest::Method::GET,
        reqwest::Url::parse("http://ascii.jp")?,
    );
    request
        .headers_mut()
        .insert("X-TEST", "ヘッダーも追加できます.".parse().unwrap());

    dbg!(&request);

    Ok(())
}
