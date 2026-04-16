pub struct HttpResponse {
    pub status: u16, 
    pub body: String,
}

impl HttpResponse {
    pub fn ok(body: &str) -> Self {
        HttpResponse {status: 200, body: body.to_string()}
    }
    pub fn not_found() -> Self {
        HttpResponse {status: 404, body: "Not Found".to_string()}
    }
    pub fn to_bytes(&self) -> String {
        let status_text = match self.status {
            200 => "OK", 
            404 => "Not Found", 
            500 => "Internal Server Error", 
            _ => "Unknown"
        }; 
        format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}", self.status, status_text, self.body.len(), self.body)
    }


}