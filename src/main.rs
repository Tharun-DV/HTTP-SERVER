use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Failed to read from stream");
    println!("Received: {}", String::from_utf8_lossy(&buf));
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
