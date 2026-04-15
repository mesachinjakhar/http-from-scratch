use std::io::Read;
use std::net::TcpListener;
use std::os::fd::AsRawFd; 
use std::io::Write;
fn main() {
    // Step 1: Create a tcp listner
   let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind address"); 

   // step 2, get access to stream on the listener 
   for (c, stream) in listener.incoming().enumerate() {
    match stream {
        Ok(mut stream) => {
            println!("stream is: {:?}", stream); // you see TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:50136, fd: 4 }, what is fd here? 
            println!("fd: {}", stream.as_raw_fd()); // this one is interesting, in mac/linux/unix , everything is treated as file, including sockets,
            // tcp connection = file, os assign a number to it = file descripter, its just a reference to the acutual socket. internally os apis are like close(fd), open(fd), read(fd). you get it. 
            // pass stream struct to handler 
            stream_handler(&mut stream); 
        }
        Err(err) => {
            println!("error {}", err);
        }
    }
   }

}

fn stream_handler(stream: &mut std::net::TcpStream) {
    let mut buffer = [0u8; 1024]; // why exactly 1024? tcp has no limit of stream, it can send any amount of bytes in one go, eg 3000 bytes, just taking 1024 bytes in one go, 
    let mut total_bytes = 0; // total bytes read in this stream
    let mut count = 0 ; // stream count 

    // create a  loop on stream
    loop {
        match stream.read(&mut buffer) { // Pull some bytes from this source into the specified buffer, returning how many bytes were read. this overwrite the buffer
            Ok(0) =>  { // 0 means, connection is closed by the client, it does not mean that connection is still alive but client is having network issue so he is not sending new data, 
                // that wrong, 0 means connection close, if client having network issue then .read function wait untill gets the data. 
                println!("connection closed"); // since stream has nothing 
                break;
            },
            Ok(n ) => {
                println!("buffer values: {:?}", buffer);
                count += 1; 
                total_bytes += n;
                // read the buffer now

                if let Ok(text) = std::str::from_utf8(&mut buffer[..n]) {
                    println!("text {}", text); 
                    if text.contains("\r\n\r\n") { // this means HTTP headers finished. 
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello";
                        stream.write_all(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                        break; // stop reading this connection
                    }

                } else {
                    println!("not a valid utf 8");
                }

            },
            Err(err) => {
                println!("error reading stream");
                break;
            }
        }
    }
    println!("total bytes :{}", total_bytes); 
    println!("total count: {}", count);
}