
**<font face="Fira Code">README</font>**
=====================================

### <font face="Fira Code">HTTP Server in Rust</font>

<font face="Fira Code" color="#d65d0e">A simple HTTP server written in Rust, serving static HTML files.</font>

**<font face="Fira Code">Table of Contents</font>**
-----------------

* <font face="Fira Code" color="#8ec07c">[Getting Started](#getting-started)</font>
* <font face="Fira Code" color="#8ec07c">[Code Explanation](#code-explanation)</font>
* <font face="Fira Code" color="#8ec07c">[Usage](#usage)</font>
* <font face="Fira Code" color="#8ec07c">[License](#license)</font>

**<font face="Fira Code">Getting Started</font>**
-----------------

<font face="Fira Code" color="#d65d0e">To run the server, simply clone the repository and run the following command:</font>

```bash
cargo run
```

<font face="Fira Code" color="#d65d0e">This will start the server on <font face="Fira Code" color="#8ec07c">http://localhost:8001</font>. Open your favorite web browser and navigate to the URL to see the server in action.</font>

**<font face="Fira Code">Code Explanation</font>**
-----------------

### <font face="Fira Code" color="#8ec07c">Importing Libraries</font>

```rust
use std::io::prelude::*;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
```

<font face="Fira Code" color="#d65d0e">Imagine you're building a Lego castle, and you need to use different types of Lego pieces. In programming, we use libraries to get access to different tools and functions that can help us build our program. The <font face="Fira Code" color="#8ec07c">use</font> keyword is like importing a box of Lego pieces into our program.</font>

* <font face="Fira Code" color="#8ec07c">std::io::prelude::*</font> is like a box of Lego pieces for input/output operations, like reading and writing files.
* <font face="Fira Code" color="#8ec07c">std::fs</font> is like a box of Lego pieces for working with files and directories.
* <font face="Fira Code" color="#8ec07c">std::net::TcpListener</font> and <font face="Fira Code" color="#8ec07c">std::net::TcpStream</font> are like boxes of Lego pieces for working with the internet and networking.

### <font face="Fira Code" color="#8ec07c">Defining a Function</font>

```rust
fn handle_stream(mut stream: TcpStream) {
    // code here
}
```

<font face="Fira Code" color="#d65d0e">A function is like a recipe for making a specific dish. You put in some ingredients (inputs), follow some steps (code), and get a result (output). In this case, our function is called <font face="Fira Code" color="#8ec07c">handle_stream</font>, and it takes one ingredient, <font face="Fira Code" color="#8ec07c">stream</font>, which is a type of Lego piece for working with the internet.</font>

### <font face="Fira Code" color="#8ec07c">Reading from the Stream</font>

```rust
let mut buf = [0; 1024];
stream.read(&mut buf).expect("Failed to read from stream");
```

<font face="Fira Code" color="#d65d0e">Imagine you're trying to read a message from a friend, but the message is written on a piece of paper that's too big to hold in your hand. You need to use a special tool (a buffer) to hold the message, and then you can read it. That's what's happening here. We're creating a buffer (a special array of 1024 zeros) to hold the message from the stream, and then we're using the <font face="Fira Code" color="#8ec07c">read</font> function to fill the buffer with the message.</font>

### <font face="Fira Code" color="#8ec07c">Checking the Message</font>

```rust
let get = b"GET / HTTP/1.1\r\n";
let (status_line, filename) =
    if buf.starts_with(b"GET / HTTP/1.1\r\n"){
        ("HTTP/1.1 200 OK\r\n","src/index.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n","src/404.html")
    };
```

<font face="Fira Code" color="#d65d0e">Imagine you're trying to understand what your friend's message says. You need to check if the message starts with a specific phrase (like <font face="Fira Code" color="#8ec07c">"GET / HTTP/1.1\r\n"</font>). If it does, you respond with a specific message (like <font face="Fira Code" color="#8ec07c">"HTTP/1.1 200 OK\r\n"</font>) and a file (like <font face="Fira Code" color="#8ec07c">"src/index.html"</font>). If it doesn't, you respond with a different message (like <font face="Fira Code" color="#8ec07c">"HTTP/1.1 404 NOT FOUND\r\n"</font>) and a different file (like <font face="Fira Code" color="#8ec07c">"src/404.html"</font>).</font>

### <font face="Fira Code" color="#8ec07c">Reading the File</font>

```rust
let content = fs::read_to_string(filename).expect("Cannot HTML read file");
```

<font face="Fira Code" color="#d65d0e">Imagine you need to read a book, but the book is stored in a special place (a file). You need to use a special tool (the <font face="Fira Code" color="#8ec07c">fs</font> library) to read the book and turn it into a string of text. That's what's happening here. We're using the <font face="Fira Code" color="#8ec07c">fs</font> library to read the file and turn it into a string of text.</font>

### <font face="Fira Code" color="#8ec07c">Creating the Response</font>

```rust
let response = format!("{}Content-Length: {}\r\n\r\n{}",status_line,content.len(),content);
```

<font face="Fira Code" color="#d65d0e">Imagine you're trying to write a response to your friend's message. You need to include the status line, the length of the content, and the content itself. That's what's happening here. We're using the <font face="Fira Code" color="#8ec07c">format!</font> function to create a string that includes all the necessary information.</font>

### <font face="Fira Code" color="#8ec07c">Writing the Response</font>

```rust
stream.write(response.as_bytes()).expect("Cannot write to stream");
stream.flush().expect("Cannot flush stream");
```

<font face="Fira Code" color="#d65d0e">Imagine you're trying to send your response to your friend. You need to write the response to the stream (like sending a letter in the mail) and then flush the stream (like sending the letter on its way). That's what's happening here. We're using the <font face="Fira Code" color="#8ec07c">write</font> function to write the response to the stream and the <font face="Fira Code" color="#8ec07c">flush</font> function to send the response on its way.</font>

### <font face="Fira Code" color="#8ec07c">Defining the Main Function</font>

```rust
fn main() {
    // code here
}
```

<font face="Fira Code" color="#d65d0e">The <font face="Fira Code" color="#8ec07c">main</font> function is like the main recipe for our program. It's where we put all the ingredients (functions) together to make the final dish (our program).</font>

### <font face="Fira Code" color="#8ec07c">Creating the TCP Listener</font>

```rust
let listener = TcpListener::bind("127.0.0.1:8001").expect("could not bind to addresss");
```

<font face="Fira Code" color="#d65d0e">Imagine you're trying to set up a phone number for your friend to call you. You need to create a special connection (a TCP listener) that listens for incoming calls. That's what's happening here. We're creating a TCP listener that listens for incoming connections on the address <font face="Fira Code" color="#8ec07c">"127.0.0.1:8001"</font>.</font>

### <font face="Fira Code" color="#8ec07c">Handling Incoming Connections</font>

```rust
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
```

<font face="Fira Code" color="#d65d0e">Imagine you're trying to answer the phone when your friend calls. You need to handle the incoming connection (the phone call) and respond accordingly. That's what's happening here. We're using a <font face="Fira Code" color="#8ec07c">for</font> loop to handle each incoming connection, and we're using a <font face="Fira Code" color="#8ec07c">match</font> statement to handle any errors that might occur.</font>

**<font face="Fira Code">Usage</font>**
-----

<font face="Fira Code" color="#d65d0e">To use the server, simply navigate to <font face="Fira Code" color="#8ec07c">http://localhost:8001</font> in your web browser. If you want to serve a different HTML file, replace <font face="Fira Code" color="#8ec07c">src/index.html</font> with the path to your file.</font>

**<font face="Fira Code">License</font>**
-------

<font face="Fira Code" color="#d65d0e">This project is licensed under the MIT License. See the <font face="Fira Code" color="#8ec07c">LICENSE</font> file for details.</font>
