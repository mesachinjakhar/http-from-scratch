use http_from_scratch::router::Router;
use http_from_scratch::request::HttpRequest;
use http_from_scratch;

fn main() {
    let mut router = Router::new();
    router.get("/", handle_index);
    router.post("/login", handle_login);
    http_from_scratch::run("127.0.0.1:8080", router);
}

fn handle_index(req: HttpRequest) -> String {
    "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello".to_string()
}

fn handle_login(req: HttpRequest) -> String {
    println!("body: {:?}", req.body);
    "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK".to_string()
}