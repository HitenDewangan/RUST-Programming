// Using Custom Error Types in Function

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

fn find_user(user_id: u32) -> Result<String, CustomError> {
    if user_id == 0 {
        return Err(CustomError::NotFound);
    } else if user_id == 1 {
        return Err(CustomError::PermissionDenied);
    } else if user_id == 2 {
        return Err(CustomError::ConnectionFailed);
    }

    Ok(String::from("User found"))
}

fn main() {
    match find_user(1) {
        Ok(user) => println!("Success: {}", user),
        Err(error) => println!("Error: {}", error),
    }

    match find_user(5) {
        Ok(user) => println!("Success: {}", user),
        Err(error) => println!("Error: {}", error),
    }
}