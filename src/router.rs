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