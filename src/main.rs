use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Failed to read from stream");
    println!("Received: {}", String::from_utf8_lossy(&buf));

    let get = b"GET / HTTP/1.1\r\n";

    if buf.starts_with(get) {

    let content = fs::read_to_string("src/index.html").expect("Cannot HTML read file");

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length {}\r\n\r\n{}", content.len(),content);
    stream
        .write(response.as_bytes())
        .expect("Cannot write to stream");
    stream.flush().expect("Cannot flush stream");
    }else{
        let content = fs::read_to_string("src/404.html").expect("Cannot HTML read file");
        let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length {}\r\n\r\n{}",content.len(),content);
        stream
            .write(response.as_bytes())
            .expect("Cannot write to stream");
        stream.flush().expect("Cannot flush stream");
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").expect("could not bind to addresss");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream);
            }
            Err(e) => {
                println!("Failed to establish connection: {}", e);
            }
        }
    }
}
