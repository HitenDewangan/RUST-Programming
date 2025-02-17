use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut file = File::create("received_file.txt")?;

    let mut buffer = [0; 1024];  // Buffer to hold incoming data, 1024 bytes 
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Connection closed by client, stop reading (EOF)
                println!("End of file reached, client closed the connection.");
                break;
            }
            Ok(bytes_read) => {
                println!("Received {} bytes", bytes_read);
                file.write_all(&buffer[0..bytes_read])?; // Write received data to the file
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e); // Log any error
                break;
            }
        }
    }

    // After receiving the file, send acknowledgement back to client
    println!("File received successfully!");
    match stream.write(b"Trnsfer complete") {
        Ok(_) => println!("Acknowledgement sent to client"),
        Err(e) => eprintln!("Error sending acknowledgement: {}", e),
    }

    Ok(())
}

fn start_server() -> std::io::Result<()> {
    // Bind the server to a specific IP and port
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080...");

    // Handle incoming client connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each client connection
                std::thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = start_server() {
        eprintln!("Error starting server: {}", e);
    }
}