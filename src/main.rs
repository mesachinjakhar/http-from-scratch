use http_from_scratch::middleware::MiddlewareResult;
use http_from_scratch::{request::HttpRequest, response::HttpResponse, router::Router};
use serde_json::json;

fn main() {
    let mut router = Router::new();
    router.use_middleware(logger); // runs first on every request
    router.use_middleware(auth); // runs second on every request
    router.get("/", handle_index);
    router.post("/login", handle_login);
    router.get("/users/:id", handle_user);
    router.get("/users/:id/posts/:post_id", handle_post);
    http_from_scratch::run("127.0.0.1:8080", router);
}

fn handle_index(req: HttpRequest) -> HttpResponse {
    HttpResponse::ok("Hello")
}
fn handle_login(req: HttpRequest) -> HttpResponse {
    HttpResponse::json(json!({
        "status": "ok",
        "message": "logged in"
    }))
}

fn handle_user(req: HttpRequest) -> HttpResponse {
    let id = req.params.get("id").unwrap();
    HttpResponse::json(json!({
        "id": id,
        "name": "John"
    }))
}

fn handle_post(req: HttpRequest) -> HttpResponse {
    let id = req.params.get("id").unwrap();
    let post_id = req.params.get("post_id").unwrap();
    HttpResponse::ok(&format!("user {} post {}", id, post_id))
}

fn logger(req: HttpRequest) -> MiddlewareResult {
    println!("[{}] {}", req.method, req.path);
    MiddlewareResult::Next(req)
}

fn auth(req: HttpRequest) -> MiddlewareResult {
    match req.headers.get("authorization") {
        Some(_) => MiddlewareResult::Next(req), // has token, continue
        None => MiddlewareResult::Respond(
            // if no token, block
            HttpResponse::new(401, "Unauthorized"),
        ),
    }
}
