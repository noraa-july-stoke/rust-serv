mod router;
mod server;
use server::{Server, Options};
// use std::io::prelude::*;
// use std::net::TcpListener;

fn main() {
    let options = Options {
        port: 8080,
        logging: true,
    };
    let server = Server::new(options);
    server.start();
}
