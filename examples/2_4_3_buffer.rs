use std::io::Write;
use std::str;

#[derive(Debug)]
enum BufferError {
    IoError(std::io::Error),
    StdError(str::Utf8Error),
}

impl From<std::io::Error> for BufferError {
    fn from(err: std::io::Error) -> Self {
        BufferError::IoError(err)
    }
}
impl From<str::Utf8Error> for BufferError {
    fn from(err: str::Utf8Error) -> Self {
        BufferError::StdError(err)
    }
}

fn main() -> Result<(), BufferError> {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.write_all(b"bytes.Buffer example1\n")?;
    println!("1\n{}", str::from_utf8(&buffer)?);
    buffer.write_all(b"bytes.Buffer example2\n")?;
    println!("2\n{}", str::from_utf8(&buffer)?);
    //                                          ^ the trait `From<Utf8Error>` is not implemented for `BufferError`
    Ok(())
}
