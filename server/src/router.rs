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
        Router { routes: vec![]}
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
        for route in &self.routes {
            if route.method == method && route.path == path {
                (route.handler)(stream, path);
                return;
            }
        }
        // If no route matches the request, send a 404 response
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
