use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::io::stdin;


fn get_file_path_from_user() -> String {
    println!("Enter the absolute path to the file you want to send:");
    let mut file_path = String::new();
    stdin().read_line(&mut file_path).expect("Failed to read line");
    file_path.trim().to_string()
}

fn send_file(file_path: &str) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server at 127.0.0.1:8080");

    let mut buffer = [0; 1024]; // Buffer to hold incoming data
    loop {
        let bytes_read = match file.read(&mut buffer) {
            Ok(0) => break, // EOF, stop reading
            Ok(bytes) => bytes,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                break;
            }
        };
        stream.write_all(&buffer[0..bytes_read])?;
    }


    // Ensure that the connection is properly closed after sending the file, (closes the client side)
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = [0; 1024]; // Buffer to hold the response

    match stream.read(&mut response) {
        Ok(bytes_read) => {
            println!("Received: {}", String::from_utf8_lossy(&response[0..bytes_read]));
        }
        Err(e) => println!("Failed reading acknowledgement: {}", e),
    }

    println!("File sent successfully!");
    Ok(())
}

fn main() {
    let file_path = get_file_path_from_user();
    if let Err(e) = send_file(&file_path) { 
        eprintln!("Error sending file: {}", e);
    }
}