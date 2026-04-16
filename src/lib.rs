use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
pub mod request;
pub mod router;

pub fn run(addr: &str, router: router::Router) {
    // Step 1: Create a tcp listner
    let listener = TcpListener::bind(addr).expect("failed to bind address");

    let router = Arc::new(router);

    // step 2, get access to stream on the listener
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let router = Arc::clone(&router);
        std::thread::spawn(move || {
            stream_handler(stream, &router);
        });
    }
}

fn stream_handler(mut stream: std::net::TcpStream, router: &router::Router) {
    let mut buffer = [0u8; 1024]; // why exactly 1024? tcp has no limit of stream, it can send any amount of bytes in one go, eg 3000 bytes, just taking 1024 bytes in one go, 
    let mut _total_bytes = 0; // total bytes read in this stream
    let mut _count = 0; // stream count 

    let mut full_request = "".to_string();
    // create a  loop on stream
    loop {
        match stream.read(&mut buffer) {
            // Pull some bytes from this source into the specified buffer, returning how many bytes were read. this overwrite the buffer
            Ok(0) => {
                // 0 means, connection is closed by the client, it does not mean that connection is still alive but client is having network issue so we are not getting new data,
                // that wrong, 0 means connection close, if client having network issue then .read function wait untill gets the data.
                println!("connection closed"); // since stream has nothing 
                break;
            }
            Ok(n) => {
                _count += 1;
                _total_bytes += n;
                // read the buffer now
                if let Ok(text) = std::str::from_utf8(&mut buffer[..n]) {
                    full_request.push_str(text);
                    // only split AFTER we've found the separator
                    if full_request.contains("\r\n\r\n") {
                        break; // headers are done, handle body separately
                    }
                } else {
                    println!("not a valid utf 8");
                }
            }
            Err(err) => {
                println!("error reading stream");
                break;
            }
        }
    }

    let (header_part, body_so_far) = full_request
        .split_once("\r\n\r\n") // devide into two parts 1: header  2: body
        .unwrap_or((&full_request, "")); // if failed keep full request as header, body : zero
    // now extract content lenght

    let content_length = header_part
        .lines()
        .find(|line: &&str| line.to_lowercase().starts_with("content-length:"))
        .and_then(|line| line.split(":").nth(1))
        .and_then(|val| val.trim().parse().ok())
        .unwrap_or(0);

    let mut body = body_so_far.to_string(); // covert &str to String

    while body.len() < content_length {
        match stream.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                if let Ok(text) = std::str::from_utf8(&buffer[..n]) {
                    body.push_str(text);
                }
            }
            Err(_) => break,
        }
    }

    let http_request = request::parse(header_part.to_string(), body);
    let response = router.dispatch(http_request);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
