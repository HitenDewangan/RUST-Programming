use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server");
            let msg = b"Hello, server";
            stream.write(msg).unwrap();
            println!("Sent: Hello, server!");

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(n) => {
                    println!("Received: {}", String::from_utf8_lossy(&buffer[0..n]));
                }
                Err(e) => println!("Failed to receive data: {}", e),
            }
        }
        Err(e) => println!("Failed to connect: {}", e),
    }
}