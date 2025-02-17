//==================================== UNIT TESTING ===============================

// pub fn multiply(a: u64, b: u64) -> u64 {
//     a * b
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_multiply() {
//     //     let result = multiply(2, 2);
//     //     assert_eq!(result, 4);
//     //     assert_eq!(result, 5);
//     // }

//     #[test]
//     fn test_assertions() {
//         // Assert that a condition is true
//         assert!(2 + 2 == 4);

//         // Assert that two values are equal
//         assert_eq!(multiply(2, 3), 6);

//         // Assert that two values are not equal
//         assert_ne!(multiply(2, 3), 7);

//         // Assert with a custom message
//         assert!(multiply(2, 2) == 4, "Multiplication failed!");
//     }
// }

// ====================================== Panic ===================================
// pub fn divide(a: i32, b: i32) -> i32 {
//     if b == 0 {
//         panic!("Division by zero!");
//     }
//     a / b
// }

// #[test]
// #[should_panic(expected = "Division by zero")] // Corrected expected message
// fn test_divide_by_zero() {
//     divide(10, 0);
// }

// ======================================  ===================================
// use std::fs;

// #[test]
// fn test_file_exists() -> Result<(), String> {
//     let file_path = "Cargo.toml";
//     if fs::metadata(file_path).is_ok() {
//         Ok(()) // Return Ok if the file exists
//     } else {
//         Err(format!("File {} does not exist.", file_path)) // Return Err if the file doesn't exist
//     }
// }

// ===================================== INTERGRATION TESTING ===================================  
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}