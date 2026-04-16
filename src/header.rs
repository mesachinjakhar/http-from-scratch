use std::collections::HashMap;

pub struct HttpRequest {
    pub method: String, 
    pub path: String, 
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

pub fn parse(header: String) {
}