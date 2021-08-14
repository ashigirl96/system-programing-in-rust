use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("./tmp/fmt.txt")?;
    writeln!(file, "{} {} {:.2}", 0, "Hello", 0.123);
    file.flush();
    Ok(())
}
