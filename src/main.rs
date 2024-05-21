use std::alloc::handle_alloc_error;
#[allow(dead_code)]

// Required Library and Modules
use std::io;
use std::io::{Bytes, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::string;
//custom types using struct
//End user
struct client;
// Server computer
struct server{
    connection : TcpListener,
};

fn read_header_line(mut stream:&TcpStream)-> io::Result<String>{
    let mut buf = Vec::with_capacity(0x1000);
    let header_line = String::new();
    match stream.bytes().next(){
        Some(Ok(byte)) => {
            if byte == b'\n'{
                return header_line;
            }
        },
        None => {
            let ioerr = io::Error::new(io::ErrorKind::ConnectionAborted, "Client aborted early");
            return Err(ioerr);    
        },
        Some(Err(err)) => return Err(err),
    }
}

impl server{
    fn new(address:&str)->server{
        let listner = TcpListener::bind(address).expect("cannot bind {address} due to some error");

        server{
            connection:listner,
        }
    }
}

enum HttpMethod{
    Get,
    Post,
    Put,
    Delete,
}
//sent request from client
struct request {
    resource:String,
    method:HttpMethod
    headers:std::collections::HashMap<String,Vec<String>>,
    body:String,
}

impl request{
    fn new(steam:TcpStream)->io::Result<request>{

        todo!()
    }
}
//sent response from server
struct response; 


fn main() {
    let server = server::new("127.0.0.1:8080");
    for stream in server.connection.incoming().flatten(){

    }
}
