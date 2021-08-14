use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()>{
    let mut file = File::create("tmp/test.txt")?;
    file.write_all(b"os.File example\n");
    Ok(())
}