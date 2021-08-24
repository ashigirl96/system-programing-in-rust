use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

// ?Sizedの説明 http://www.nct9.ne.jp/m_hiroi/linux/rustabc03.html
fn copy_n<R: ?Sized + Read, W: ?Sized + Write>(
    reader: &mut R,
    writer: &mut W,
    size: u64,
) -> io::Result<u64> {
    io::copy(&mut reader.take(size), writer)
}

fn main() -> io::Result<()> {
    // ReaderのすべてのデータをWriterに渡したいなあ
    let mut reader = File::open("./Cargo.toml")?;
    // let mut writer = BufWriter::new(io::stdout());
    let mut writer = File::create("./tmp/Cargo.fake.toml")?;
    io::copy(&mut reader, &mut writer)?;

    println!("{}", "=".repeat(20));

    let mut reader = File::open("./Cargo.lock")?;
    let mut writer_n = BufWriter::with_capacity(8, io::stdout());
    copy_n(&mut reader, &mut writer_n, 8)?;
    Ok(())
}
