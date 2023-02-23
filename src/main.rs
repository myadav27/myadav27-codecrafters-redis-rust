use std::{io::{BufRead, BufReader, Write}, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let reader = BufReader::new(&stream);
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            if line == "PING\r\n" {
                                let response = "+PONG\r\n".as_bytes();
                                stream.write_all(&response).unwrap();
                            }
                        }
                        Err(e) => {
                            println!("error reading line: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
