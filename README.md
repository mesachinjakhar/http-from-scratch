# http-from-scratch
 
A simple HTTP server library built from raw TCP in Rust. No frameworks, no dependencies except `serde_json`. Just sockets, threads, and the HTTP spec.
 
I built this to understand how web frameworks like Axum actually work under the hood.
 
## Features
 
- HTTP request parsing (method, path, headers, body)
- Router with GET and POST support
- Path parameters (`/users/:id`)
- Query string parsing (`/search?q=rust`)
- JSON responses
- Middleware support (logging, auth, etc)
- Thread pool for concurrent connections
- Request timeouts
## Usage
 
Add to your `Cargo.toml`:
 
```toml
[dependencies]
http-from-scratch = { path = "." }
serde_json = "1"
```
 
Basic example:
 
```rust
use http_from_scratch::router::Router;
use http_from_scratch::request::HttpRequest;
use http_from_scratch::response::HttpResponse;
use serde_json::json;
 
fn main() {
    let mut router = Router::new();
 
    router.get("/", handle_index);
    router.get("/users/:id", handle_user);
    router.post("/login", handle_login);
 
    http_from_scratch::run("127.0.0.1:8080", router);
}
 
fn handle_index(req: HttpRequest) -> HttpResponse {
    HttpResponse::ok("Hello!")
}
 
fn handle_user(req: HttpRequest) -> HttpResponse {
    let id = req.params.get("id").unwrap();
    HttpResponse::json(json!({ "id": id }))
}
 
fn handle_login(req: HttpRequest) -> HttpResponse {
    println!("body: {:?}", req.body);
    HttpResponse::json(json!({ "status": "ok" }))
}
```
 
## Middleware
 
```rust
use http_from_scratch::middleware::MiddlewareResult;
 
fn logger(req: HttpRequest) -> MiddlewareResult {
    println!("[{}] {}", req.method, req.path);
    MiddlewareResult::Next(req)
}
 
fn auth(req: HttpRequest) -> MiddlewareResult {
    match req.headers.get("authorization") {
        Some(_) => MiddlewareResult::Next(req),
        None => MiddlewareResult::Respond(HttpResponse::new(401, "Unauthorized")),
    }
}
 
fn main() {
    let mut router = Router::new();
    router.use_middleware(logger);
    router.use_middleware(auth);
    // ...
}
```
 
## How it works
 
Most people use Axum or Actix without thinking about what happens below. This project goes one level down:
 
1. bind a TCP socket to a port
2. accept incoming connections
3. read raw bytes from the socket
4. parse those bytes into an HTTP request manually
5. match the request against registered routes
6. call the handler and write the response back to the socket
Each connection gets its own thread from a fixed thread pool so the server stays responsive under load.
 
## Project structure
 
```
src/
  lib.rs          - server entry point, thread pool, connection handling
  request.rs      - HTTP request parsing
  response.rs     - HTTP response builder
  router.rs       - route registration and dispatch
  middleware.rs   - middleware chain
```
 
## Why
 
I wanted to know what actually happens when a request hits a web server. Reading docs only goes so far. Building it from scratch made it stick.
 
## License
 
MIT
