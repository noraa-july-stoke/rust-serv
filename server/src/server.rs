use crate::router::Router;
use std::io::prelude::*;
use std::net::TcpListener;

pub struct Options {
    pub port: u16,
    pub logging: bool,
}

pub struct Server {
    router: Router,
    options: Options,
}

impl Server {
    pub fn new(options: Options) -> Server {
        Server {
            router: Router::new(),
            options,
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.options.port)).unwrap();
        println!("🪴 Server available @ port {}...", self.options.port);

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

            if self.options.logging {
                let message = format!("Received {} request for {}", method, path);
                println!("{}", message);
            }

            self.router.handle_request(&mut stream, method, path);
        }
    }
}

// pub fn start(config: Options, router: Router) {
//     let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
//     println!("Listening on port {}", config.port);

//     for stream in listener.incoming() {
//         match stream {
//             Ok(mut stream) => {
//                 let mut buffer = [0; 1024];
//                 stream.read(&mut buffer).unwrap();

//                 let request = String::from_utf8_lossy(&buffer[..]);
//                 let request_lines: Vec<&str> = request.lines().collect();

//                 let request_line = request_lines[0];
//                 let request_parts: Vec<&str> = request_line.split_whitespace().collect();
//                 let method = request_parts[0];
//                 let path = request_parts[1];

//                 router.handle_request(&mut stream, method, path);
//             }
//             Err(e) => {
//                 eprintln!("Error: {}", e);
//             }
//         }
//     }
// }
