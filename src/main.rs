use std::io;
use std::net::{TcpListener, TcpStream};

/// End User
struct Clinet;

/// Computer hosting the Web App
struct Server {
    connection: TcpListener,
}

impl Server {
    fn new(address: &str) -> Server {
        let listner: TcpListener = TcpListener::bind(address).unwrap();

        Server {
            connection: listner,
        }
    }
}

enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    TRACE,
    CONNECT,
}
///Sent From Client
struct Request {
    /// route to the resource
    resource: String,

    /// HTTP Method like get , post , put , delete and etc...
    method: HTTPMethod,

    /// headers
    headers: std::collections::HashMap<String, Vec<String>>,

    /// body
    body: String,
}
impl Request {
    fn new(stream: TcpStream) -> io::Result<Request> {
        unimplemented!()
    }
}

/// Sent From Server
struct Response;

fn main() {
    let server: Server = Server::new("127.0.0.1:8080");
    for stream in server.connection.incoming().flatten() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
