use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move ||handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("{:?}", stream);
    loop {
        let mut buf = [0;100];
        let line = stream.read(&mut buf).unwrap);
       // let line = match stream.read(&mut buf) {
       //     Ok(v) => v,
       //     Err(e) => {
       //         println!("Error: {}", e);
       //         break;
       //     }
       // };
        stream.write(&buf[0..line]);
    }

   // for line in reader.by_ref().lines() {
   //     let line = match line {
   //         Ok(v) => v,
   //         Err(e) => {
   //             println!("Error: {}", e);
   //             break;
   //         }
   //     };
   //     if line == "" {
   //         break;
   //     }
   // }
   // let response = "HTTP/1.1 200 OK\n\n<html><body>Hello, world!</body></html>";
   // reader.into_inner().write_all(response.as_bytes()).unwrap();
}
