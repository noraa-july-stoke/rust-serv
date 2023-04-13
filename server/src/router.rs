use std::io::Write;
use std::net::TcpStream;

pub struct Router {
    routes: Vec<Route>,
}
pub struct Route {
    method: String,
    path: String,
    handler: fn(&mut TcpStream, &str),
}

impl Router {
    pub fn new() -> Self {
        Router { routes: vec![] }
    }

    #[allow(dead_code)]
    pub fn get(&mut self, path: &str, handler: fn(&mut TcpStream, &str)) {
        self.add_route("GET", path, handler);
    }

    #[allow(dead_code)]
    pub fn post(&mut self, path: &str, handler: fn(&mut TcpStream, &str)) {
        self.add_route("POST", path, handler);
    }

    #[allow(dead_code)]
    pub fn put(&mut self, path: &str, handler: fn(&mut TcpStream, &str)) {
        self.add_route("PUT", path, handler);
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, path: &str, handler: fn(&mut TcpStream, &str)) {
        self.add_route("DELETE", path, handler);
    }

    pub fn add_route(&mut self, method: &str, path: &str, handler: fn(&mut TcpStream, &str)) {
        let route = Route {
            method: method.to_string(),
            path: path.to_string(),
            handler,
        };
        self.routes.push(route);
    }

    pub fn handle_request(&self, stream: &mut TcpStream, method: &str, path: &str) {
        println!("handling request for {}: {}", method, path);
        for route in &self.routes {
        println!("checking route {} {}", route.method, route.path);
            if route.method == method && route.path == path {
                println!("found handler for route {} {}", route.method, route.path);
                (route.handler)(stream, path);
                return;
            }
        }
        // If no route matches the request, send a 404 response
        println!("no handler found for {}: {}", method, path);
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
