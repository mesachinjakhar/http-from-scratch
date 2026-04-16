use std::collections::HashMap;

pub struct HttpRequest {
    pub method: String, 
    pub path: String, 
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

pub fn parse(header: String, body: String) {
    println!("headers: {}", header);
    println!("body: {}", body);
    let mut lines = header.lines(); 

    let request_line = lines.next().unwrap_or(""); 
    let mut parts = request_line.split_whitespace(); 

    let method = parts.next().unwrap_or("").to_string(); 
    let path = parts.next().unwrap_or("").to_string();
    let version = parts.next().unwrap_or("").to_string();
    

}