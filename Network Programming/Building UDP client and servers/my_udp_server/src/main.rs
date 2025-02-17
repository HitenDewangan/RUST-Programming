// UPD Server

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    // let mut socket = match socket {
    //     Ok(socket): UdpSocket => socket,
    //     Err(e) => {
    //         println!("Failed to bind socket: {}", e);
    //         return Err(e);
    //     }
    // };
    println!("UDP Server listening on 127.0.0.1:8080");

    let mut buffer = [0; 512];

    loop {
        // Receive data from the client
        let (bytes_received, src_addr) = socket.recv_from(&mut buffer)?;
        println!(
            "Received {} bytes from {}: {}", 
            bytes_received, 
            src_addr, 
            String::from_utf8_lossy(&buffer[..bytes_received]));

        // Echo the data back to the client
        let response = "Hello, client!";
        socket.send_to(response.as_bytes(), &src_addr)?;
        println!("Sent response to {}", src_addr);
    }
}



// ? : denotes that the value may be an error