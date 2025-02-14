// Combining Multiple Error Types into a Single Custom Error Type

use std::fmt;

// Define the NetworkError enum
#[derive(Debug)]
enum NetworkError {
    Disconnected,
    Timeout,
}

// Define the CustomError enum, which includes a Network error
#[derive(Debug)]
enum CustomError {
    NotFound,
    PermissionDenied,
    ConnectionFailed,
    Network(NetworkError), // Wraps NetworkError
}

// Implement Display for NetworkError
impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Disconnected => write!(f, "Network disconnected"),
            NetworkError::Timeout => write!(f, "Network timeout"),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "Resource not found"),
            CustomError::PermissionDenied => write!(f, "Permission denied"),
            CustomError::ConnectionFailed => write!(f, "Connection failed"),
            CustomError::Network(e) => write!(f, "Network error: {}", e), 
        }
    }
}

impl std::error::Error for NetworkError {} // Implement Error for NetworkError
impl std::error::Error for CustomError {} // Implement Error for CustomError


// Implement the From trait for converting NetworkError into CustomError
impl From<NetworkError> for CustomError {
    fn from(error: NetworkError) -> Self {
        CustomError::Network(error) // Wraps NetworkError inside CustomError::Network
    }
}

// Simulating a function that returns a NetworkError
fn connect_to_network() -> Result<(), NetworkError> {
    Err(NetworkError::Disconnected) // Simulating a network error
}

// Function that uses the From trait to convert NetworkError into CustomError
fn perform_task() -> Result<(), CustomError> {
    // 'connect_to_network' returns Result<(), NetworkError>.
    // The '?' operator will convert NetworkError into CustomError using the From trait.
    connect_to_network()?;
    Ok(())
}

fn main() {
    match perform_task() {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => println!("Task failed: {}", e),
    }
}