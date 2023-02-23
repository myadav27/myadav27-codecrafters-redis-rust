use std::{io::Write, net::TcpListener};

use tokio::stream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = "+PONG\r\n".as_bytes();
                stream.write_all(&response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
