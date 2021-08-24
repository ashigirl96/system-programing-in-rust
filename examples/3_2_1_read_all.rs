use std::fs::File;
use std::io::{self, Read};
use std::str;

fn main() -> std::io::Result<()> {
    let mut buffer: [u8; 6] = [0; 6];
    let mut reader = File::open("./Cargo.toml")?;
    reader.read_exact(&mut buffer);
    println!("> {:?}", str::from_utf8(&buffer));

    let mut string = String::new();
    reader.read_to_string(&mut string);
    println!("> {}", string);
    Ok(())
}
