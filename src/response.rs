use serde_json::Value;
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    content_type: String,
}

impl HttpResponse {
    pub fn ok(body: &str) -> Self {
        HttpResponse {
            status: 200,
            body: body.to_string(),
            content_type: "text/plain".to_string(),
        }
    }
    pub fn not_found() -> Self {
        HttpResponse {
            status: 404,
            body: "Not Found".to_string(),
            content_type: "text/plain".to_string(), // ← add this
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
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            self.status_text(),
            self.content_type,
            self.body.len(),
            self.body
        )
    }

    pub fn json(data: Value) -> Self {
        HttpResponse {
            status: 200,
            body: data.to_string(),
            content_type: "application/json".to_string(),
        }
    }
}
