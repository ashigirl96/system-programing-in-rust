#![feature(box_syntax)]

use flate2::{write::GzEncoder, Compression};
use lib::io::MultiWriter;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("./tmp/multi_writer.txt")?;
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut writer = MultiWriter::new(vec![box (file), box (stdout), box (stderr)]);
    writer.write_all(b"MultiWriter Example\n")?;

    let file2 = File::create("./tmp/test.txt.gz")?;
    let mut writer2 = GzEncoder::new(file2, Compression::default());
    writer2.write_all(b"MultiWriter Example\n");

    let mut buffer = BufWriter::new(std::io::stdout());
    buffer.write_all(b"1\n")?;
    buffer.write_all(b"2\n")?;
    buffer.flush(); // バッファの出力をする
    println!("Interpreter1");
    buffer.write_all(b"3\n")?;
    buffer.write_all(b"4\n")?;
    println!("Interpreter2");
    // closeするので、溜まってるバッファを出力する

    Ok(())
}
