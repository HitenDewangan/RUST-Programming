use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Function to handle communication with a client
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // Buffer to hold incoming data
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed by client
            Ok(n) => {
                stream.write_all(&buffer[0..n]).unwrap(); // Echo back the received data (corrected write)
                println!("Received: {}", String::from_utf8_lossy(&buffer[0..n])); // Log received data
            }
            Err(e) => {
                println!("Error reading stream: {}", e); // Log any error
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // Bind to localhost port 8080
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: Connection Failed: {}", e);
            }
        }
    }

    println!("Hello, world!"); // This will be printed after the loop (never reached in this example)
}