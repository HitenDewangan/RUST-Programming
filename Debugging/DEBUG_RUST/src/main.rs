// fn util() {
//     println!("This is a utility function");
// }

// fn main() {
//     let x: i32 = 5;
//     println!("The value of x is: {}", x);

//     let y: i32 = 10;
//     println!("The value of y is: {}", y);

//     let z: i32 = x + y;
//     println!("x + y = {}", z);

//     util()
// }

// =========================== Logging in Rust ===================================
// priority of log levels: trace < debug < info < warn < error

// use log::{debug, error, info, trace, warn}; // Improting logging macros
// use env_logger;

// fn main() {
//     env_logger::init(); // Initialize the logger

//     // debug!("This is a debug message");
//     info!("This is an info message");
//     warn!("This is a warning message");
//     error!("This is an error message");
//     trace!("This is a trace message");
// }

// =============================================================================
// use log::{debug, error, info, warn};

// fn main() {
//     env_logger::init(); // Initialize the logger

//     info!("Application started");

//     let config_loaded: bool = false; // Simulate whether a config file is loaded

//     if !config_loaded { // Corrected condition: if NOT config_loaded
//         warn!("Configuration not loaded; using defaults");
//     }

//     let result: Result<i32, String> = divide(10, 0); // Corrected type and values

//     match result {
//         Ok(value) => info!("Division successful: {}", value),
//         Err(e) => error!("Failed to divide: {}", e),
//     }

//     debug!("Application finished");
// }

// fn divide(a: i32, b: i32) -> Result<i32, String> { // Corrected type
//     if b == 0 { // Corrected comparison: == for equality
//         return Err(String::from("Division by zero"));
//     }
//     Ok(a / b)
// }