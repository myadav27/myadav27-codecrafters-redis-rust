use std::io::{Read, Write};
use std::net::TcpListener;
// use std::{io::Write, net::TcpListener};

use tokio::stream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    #[allow(clippy::unused_io_amount)]
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = [0; 512];
                loop {
                    let bytes_read = stream.read(&mut buf).unwrap();
                    if bytes_read == 0 {
                        println!("client closed the connection");
                        break;
                    }

                    stream.write("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
