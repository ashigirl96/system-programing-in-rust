use std::io::{self, copy, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("ascii.jp:80")?;
    let request = "GET / HTTP/1.0\r\nHost: ascii.jp\r\n\r\n";
    stream.write_all(request.as_bytes());
    let mut stdout = io::stdout();
    copy(&mut stream, &mut stdout);
    Ok(())
}
