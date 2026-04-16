use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String, 
    pub path: String, 
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

pub fn parse(header: String, body: String) -> HttpRequest {
    let mut lines = header.lines(); 

    let request_line = lines.next().unwrap_or(""); 
    let mut parts = request_line.split_whitespace(); 

    let method = parts.next().unwrap_or("").to_string(); 
    let path = parts.next().unwrap_or("").to_string();
    let version = parts.next().unwrap_or("").to_string();

    // parse remaining lines key : value; eg: Host: localhost:8080 , User-Agent: insomnia/12.4.0

    let mut headers = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(":") {
            headers.insert(key.trim().to_lowercase(),value.to_string());
        }
    }

    let body = if body.is_empty() {
        None
    } else {
        Some(body)
    };

    HttpRequest {method, path, version, headers, body}

}