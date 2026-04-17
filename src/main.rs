use http_from_scratch::{request::HttpRequest, router::Router, response::HttpResponse};

fn main() {
    let mut router = Router::new();
    router.get("/", handle_index);
    router.post("/login", handle_login);
    http_from_scratch::run("127.0.0.1:8080", router);
}

fn handle_index(req: HttpRequest) -> HttpResponse {
    HttpResponse::ok("Hello")
}

fn handle_login(req: HttpRequest) -> HttpResponse {
    HttpResponse::ok("Logged in")
}
