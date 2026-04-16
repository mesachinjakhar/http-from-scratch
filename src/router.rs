use std::collections::HashMap;
use crate::request::HttpRequest;
// any function that takes HttpRequest and returns a String response
type Handler = fn(HttpRequest) -> String; 

pub struct Router {
    routes: HashMap<(String, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn get(&mut self, path: &str, hander: Handler) {
        self.routes.insert(("GET".to_string(), path.to_string()), hander);
    }
    pub fn post(&mut self, path: &str, hander: Handler) {
        self.routes.insert(("GET".to_string(), path.to_string()), hander);
    }
    pub fn dispatch(&self, request: HttpRequest) -> String {
        let key = (request.method.clone(), request.path.clone());
        match self.routes.get(&key) {
            Some(Handler) => Handler(request),
            None => "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found".to_string(),
        }
    }
}

fn handle_index(req: HttpRequest) -> String {
    "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello".to_string()
}

fn handle_login(req: HttpRequest) -> String {
    println!("body: {:?}", req.body);
    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK".to_string()
}

fn main() {
    let mut router = Router::new();
    router.get("/", handle_index);
    router.post("/login", handle_login);

    // pass router into stream_handler
}