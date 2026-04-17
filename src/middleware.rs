use crate::request::HttpRequest;
use crate::response::HttpResponse;

pub enum MiddlewareResult {
    Next(HttpRequest),
    Respond(HttpResponse),
}

pub type Middleware = fn(HttpRequest) -> MiddlewareResult;
