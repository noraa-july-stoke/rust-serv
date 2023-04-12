use std::io::prelude::*;
use std::net::TcpListener;

struct Router {
    routes: Vec<Route>,
}

struct Route {
    method: String,
    path: String,
    handler: fn(&mut TcpStream, &str),
}

impl Router {
    fn new() -> Self {
        Rotuer { routes: vec![]}
    }

    fn add_route(&mut self, method: &str, path: &str, handler: fn(&mut TcpStream, &str)) {
        let route = Route {
            method: method.to_string(),
            path: path.to_string(),
            handler,
        };

        self.routes.push(route);
    }

    fn handle_request(&self, stream: &mut TcpStream, method: &str, path: &str) {
        for route in &self.routes {
            if route.method == method && route.path == path {
                (route.handler)(stream, path);
                return;
            }
        }

    }
}
