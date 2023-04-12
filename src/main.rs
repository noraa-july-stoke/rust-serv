use std::io::prelude::*;
use std::net::TcpListener;
// use std::net::TcpStream;

fn main () {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
