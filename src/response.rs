pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

impl HttpResponse {
    pub fn ok(body: &str) -> Self {
        HttpResponse {
            status: 200,
            body: body.to_string(),
        }
    }
    pub fn not_found() -> Self {
        HttpResponse {
            status: 404,
            body: "Not Found".to_string(),
        }
    }

    fn status_text(&self) -> &str {
        match self.status {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            400 => "Bad Request",
            _ => "Unknown",
        }
    }

    pub fn to_http_string(&self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            self.status_text(),
            self.body.len(),
            self.body
        )
    }
}
