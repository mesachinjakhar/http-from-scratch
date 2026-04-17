use http_from_scratch::{request::HttpRequest, response::HttpResponse, router::Router};

fn main() {
    let mut router = Router::new();
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
    HttpResponse::ok("Logged in")
}


fn handle_user(req: HttpRequest) -> HttpResponse {
    let id = req.params.get("id").unwrap();
    HttpResponse::ok(&format!("user id is {}", id))
}

fn handle_post(req: HttpRequest) -> HttpResponse {
    let id      = req.params.get("id").unwrap();
    let post_id = req.params.get("post_id").unwrap();
    HttpResponse::ok(&format!("user {} post {}", id, post_id))
}