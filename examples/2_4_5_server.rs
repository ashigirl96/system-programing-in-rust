use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn get_operation(stream: &mut TcpStream) -> io::Result<()> {
    let body = "HTTP Server sample";
    writeln!(stream, "HTTP/1.1 200 OK")?;
    writeln!(stream, "Content-Type: text/plain; charset=UTF-8")?;
    writeln!(stream, "Content-Length: {}", body.len())?;
    writeln!(stream)?;

    writeln!(stream, "{}", body)?;
    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);

    let mut first_line = String::new();
    reader.read_line(&mut first_line);

    // first_line = "GET / HTTP/1.1\r\n"
    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    match (method, path) {
        (Some("GET"), Some(_)) => {
            get_operation(reader.get_mut())?;
        }
        _ => panic!("Failed to parse"),
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // loop {
    //     let (mut stream, _) = listener.accept()?;
    //     thread::spawn(move || {
    //         handle_client(stream);
    //     });
    // }
    for (i_client, stream) in listener.incoming().enumerate() {
        println!("Client {} incoming", i_client);
        handle_client(stream?)?;
    }
    Ok(())
}
