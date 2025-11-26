use std::{io::Read, net::TcpListener};


fn main() {
   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

loop {
    let  ( mut stream, addr) = listener.accept().unwrap();
    let mut buf = [0; 1024];

    loop  {
        let n = match  stream.read(& mut buf) {
            Ok(n) => n,
            Err(e) => {
                println!("Read error: {e}");
                break;
            }
        };
        if n == 0 {
            break;
        }
        println!("{:?}", buf);
    }

}}
