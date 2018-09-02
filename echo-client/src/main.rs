use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::io::prelude::*;

fn main() {
    match TcpStream::connect("127.0.0.1:17007") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 17007" );
            let stdin = std::io::stdin();
            for line_result in stdin.lock().lines() {
                let mut line = line_result.unwrap();
                stream.write(line.as_ref()).unwrap();
                let mut data = [0 as u8; 10];
                match stream.read_exact(&mut data) {
                    Ok(a) => {
                        println!("{:?}", a);
                    }
                    Err(e) => {
                        println!("{}",e);
                    }
                }
            }
        },
        Err(e) => println!("Failed to connect: {}", e),
    }
}
