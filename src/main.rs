use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
   let listener = TcpListener::bind("127.0.0.1:8080")
   .expect("Failed to bind the address");

   println!("Server is listening on http://127.0.0.1:8080");
   println!("Waiting for connections...\n");

   // Step 2: Accept loop - handle one connection at a time
   for (connection_num, stream) in listener.incoming().enumerate() {
    match stream {
        Ok(mut stream) => {
            println!("═══════════════════════════════════════");
            println!("Connection {} accepted", connection_num + 1);
            println!("From: {}", stream.peer_addr().unwrap());
            println!("═══════════════════════════════════════");

            // Step 3: Handle Connection
            handle_connection(&mut stream, connection_num + 1);

        }
        Err(e) => {
            println!("Error accepting connection: {}", e);
        }
    }
   }

}

fn  handle_connection(stream: &mut std::net::TcpStream, conn_num: usize) {
    let mut buffer = [0u8; 1024];
    let mut total_bytes = 0;
    let mut read_count = 0;

    println!("Reading from stream");

    // Step 4: Read stream untill connection closed
    loop { // read method overwrite the old values 
        match stream.read(&mut buffer) {
            // read method return the result containing the total number of bytes written to buffer in this iteration. eg: 5
            Ok(0) => {
                println!("Connection closed ");
                break;
            }
            Ok(n) => {
                read_count +=1;
                total_bytes += n;
                println!("📦 Read #{}: {} bytes", read_count, n);

                // we extract info into text from the current total bytes recieved
                if let Ok(text) = std::str::from_utf8(&mut buffer[..n]) {
                    println!("As text: {:?}", text);
                } else {
                    println!("(not valid UTF-8)");
                }

            }
            Err(e) => {
                println!("Error reading stream {}", e);
                break;
            }
        }
    }

    println!("📊 Statistics:");
    println!("   Total reads: {}", read_count);
    println!("   Total bytes: {}", total_bytes);
}