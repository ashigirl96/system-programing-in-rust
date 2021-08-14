#![feature(box_syntax)]

use flate2::read::GzEncoder;
use flate2::Compression;
use lib::io::MultiWriter;
use serde_json::{json, to_writer_pretty};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle(stream?);
    }
    Ok(())
}

fn handle(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut request = String::new();
    reader.read_line(&mut request);

    let gzip_encoder = GzEncoder::new(stream, Compression::default());
    let mut writer = MultiWriter::new(vec![box (gzip_encoder), box (std::io::stdout())]);

    if let ["GET", "/", ..] = request.split_whitespace().collect::<Vec<_>>().as_slice() {
        let source = json!({
            "Hello": "World",
        });
        writeln!(writer, "HTTP/1.1 200 OK")?;
        writeln!(writer, "Content-Encoding: gzip")?;
        writeln!(writer, "Content-Type: application/json")?;
        writeln!(writer)?;
        to_writer_pretty(writer, &source)?;
    } else {
        eprintln!("Cannot parse request: {}", request);
    }
    Ok(())
}
