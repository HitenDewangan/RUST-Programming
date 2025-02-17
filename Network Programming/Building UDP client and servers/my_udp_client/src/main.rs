use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8081")?; // Bind to a different port for the client
    println!("UDP Client started");

    let server_addr = "127.0.0.1:8080"; // Server address and port
    let message = "Hello, server!, from UDP client!";

    socket.send_to(message.as_bytes(), server_addr)?;
    println!("Sent message to {}: {}", server_addr, message);

    let mut buffer = [0; 512];
    let (bytes_received, src_addr) = socket.recv_from(&mut buffer)?; // Type annotation not needed here

    println!(
        "Received from server {}: {}",
        src_addr,
        String::from_utf8_lossy(&buffer[..bytes_received])
    );

    Ok(()) // Correct placement of Ok(())
}