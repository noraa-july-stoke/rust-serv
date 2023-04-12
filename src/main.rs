mod router;
use std::io::prelude::*;
use std::net::TcpListener;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.add_route("GET", "/", |stream, _| {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);

        let request_lines: Vec<&str> = request.lines().collect();
        let request_line = request_lines[0];
        let request_parts: Vec<&str> = request_line.split(" ").collect();
        let method = request_parts[0];
        let path = request_parts[1];

        router.handle_request(&mut stream, method, path);
    }
}
