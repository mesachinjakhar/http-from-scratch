use crate::middleware::{Middleware, MiddlewareResult};
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use std::collections::HashMap;

// any function that takes HttpRequest and returns a String response
type Handler = fn(HttpRequest) -> HttpResponse;

pub struct Router {
    routes: Vec<(String, String, Handler)>, // (method, pattern, handler)
    middlewares: Vec<Middleware>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }
    pub fn use_middleware(&mut self, m: Middleware) {
        self.middlewares.push(m);
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.routes
            .push(("GET".to_string(), path.to_string(), handler));
    }
    pub fn post(&mut self, path: &str, handler: Handler) {
        self.routes
            .push(("POST".to_string(), path.to_string(), handler));
    }
    pub fn dispatch(&self, mut request: HttpRequest) -> HttpResponse {
        // run through every middleware in order
        for middleware in &self.middlewares {
            match middleware(request) {
                MiddlewareResult::Next(req) => {
                    request = req; // passed, continue with (possibly modified) request
                }
                MiddlewareResult::Respond(response) => {
                    return response; // short-circuited, skip handler entirely
                }
            }
        }

        // middleware chain passed, now find the route
        for (method, pattern, handler) in &self.routes {
            if method != &request.method {
                continue;
            }
            if let Some(params) = match_path(pattern, &request.path) {
                request.params = params;
                return handler(request);
            }
        }

        HttpResponse::not_found()
    }
}

// returns Some(params) if pattern matches path, None if not
fn match_path(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let mut params = HashMap::new();

    let pattern_parts: Vec<&str> = pattern.split('/').collect();
    let path_parts: Vec<&str> = path.split('/').collect();

    // different number of segments = no match
    if pattern_parts.len() != path_parts.len() {
        return None;
    }

    for (pattern_seg, path_seg) in pattern_parts.iter().zip(path_parts.iter()) {
        if pattern_seg.starts_with(':') {
            // this s a param, capture iti
            let key = pattern_seg.trim_start_matches(':');
            params.insert(key.to_string(), path_seg.to_string());
        } else if pattern_seg != path_seg {
            // static segment doesn't match
            return None;
        }
    }

    Some(params)
}
