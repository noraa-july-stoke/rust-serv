mod router;
mod server;
use server::{Server, Options};
use router::Router;
use std::io::prelude::*;
// use std::net::TcpListener;

fn main() {
    let options = Options {
        port: 8080,
        logging: true,
    };


    let mut test_router = Router::new();

    test_router.get("/", | stream, _ | {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    test_router.get("/yo", | stream, _ | {
        let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    let server = Server::new(options);
    server.start();

    // Test router



}
