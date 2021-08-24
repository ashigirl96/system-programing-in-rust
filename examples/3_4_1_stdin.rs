use std::io::{self, stdin, Read};

// e.g. cargo run --example 3_4_1_stdin < Cargo.toml
fn main() -> io::Result<()> {
    loop {
        let mut buffer: Vec<u8> = vec![0; 5];
        let size = stdin().read(&mut buffer[..])?;
        if size == 0 {
            println!("EOF");
            break;
        }
        println!("size={} input={}", size, String::from_utf8(buffer).unwrap());
    }
    Ok(())
}
