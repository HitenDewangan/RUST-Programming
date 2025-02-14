// Implementing the Error and Display Trait

use std::fmt;

#[derive(Debug)]
enum CustomError {
    NotFound,
    PermissionDenied,
    ConnectionFailed,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotFound => write!(f, "Resource not found"),
            CustomError::PermissionDenied => write!(f, "Permission denied"),
            CustomError::ConnectionFailed => write!(f, "Connection failed"),
        }
    }
}

impl std::error::Error for CustomError {}

fn main() {
    let error = CustomError::NotFound;
    println!("{}", error);
}