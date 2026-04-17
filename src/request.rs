use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub fn parse(header: String, body: String) -> HttpRequest {
    let mut lines = header.lines();

    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("").to_string();
    let raw_path = parts.next().unwrap_or("").to_string();
    let version = parts.next().unwrap_or("").to_string();

    let (path, query) = parse_path(&raw_path);

    // parse remaining lines key : value; eg: Host: localhost:8080 , User-Agent: insomnia/12.4.0

    let mut headers = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(":") {
            headers.insert(key.trim().to_lowercase(), value.to_string());
        }
    }

    let body = if body.is_empty() { None } else { Some(body) };
    let params = HashMap::new(); // ← empty here, router fills it later

    HttpRequest {
        method,
        path,
        query,
        params,
        version,
        headers,
        body,
    }
}

fn parse_path(raw_path: &str) -> (String, HashMap<String, String>) {
    let mut query = HashMap::new();

    // split at '?' → ["/search", "q=rust&page=2"]
    match raw_path.split_once('?') {
        None => {
            // no query string at all
            (raw_path.to_string(), query)
        }
        Some((path, query_string)) => {
            // "q=rust&page=2" → ["q=rust", "page=2"]
            for pair in query_string.split('&') {
                if let Some((key, value)) = pair.split_once('=') {
                    query.insert(key.to_string(), value.to_string());
                }
            }
            (path.to_string(), query)
        }
    }
}
