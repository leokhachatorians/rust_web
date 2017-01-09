use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(stream: TcpStream) {
    thread::spawn(|| {
        let stream = stream;
        println!("{:?}", stream);
        let mut reader = BufReader::new(stream);

        for line in reader.by_ref().lines() {
            let line = match line {
                Ok(v) => v,
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                    //std::process::exit(1);
                }
            };
            if line == "" {
                break;
            }
        }
        let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, world!</body></html>";
        reader.into_inner().write_all(response.as_bytes()).unwrap();
    });
}
