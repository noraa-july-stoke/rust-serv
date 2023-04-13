mod router;
use router::Router;
use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let mut router = Router::new();

    router.get("/", |stream, _| {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    router.post("/", |stream, _body| {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    router.put("/", |stream, _body| {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    router.delete("/", |stream, _| {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        let message = "The server has been hit!";
        println!("{}", message);

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
