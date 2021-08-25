use lib::io::{LimitedReader, SectionReader};
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // let f1 = create_read_write_temp_file("Example of io.SectionReader")?;
    let mut section_reader = SectionReader::new(File::open("./Cargo.toml").unwrap(), 2, 3)?;
    dbg!(&section_reader);

    // TODO: Vec<u8>だと5バイト分読みこんでしまう
    // let mut buf: Vec<u8> = vec![0; 5];
    // let read_bytes = section_reader.read(&mut buf[..])?;
    // dbg!(String::from_utf8(buf));
    let mut buf = String::new();
    let read_bytes = section_reader.read_to_string(&mut buf)?;
    dbg!(buf);
    dbg!(&read_bytes);

    let mut limited_reader = LimitedReader::new(File::open("./Cargo.toml").unwrap(), 3)?;
    let mut buf = String::new();
    let read_bytes = limited_reader.read_to_string(&mut buf)?;
    dbg!(buf);
    dbg!(&read_bytes);
    Ok(())
}
