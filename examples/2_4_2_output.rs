use std::io::Write;

fn main() -> std::io::Result<()>{
    std::io::stdout().write_all(b"os.Stdout example\n")?;
    std::io::stderr().write_all(b"os.Stderr example\n")?;
    Ok(())
}