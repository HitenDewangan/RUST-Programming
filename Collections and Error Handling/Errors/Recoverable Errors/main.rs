// Function that returns Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero error"))
    } else {
        Ok(a / b)
    }
}

// Function that returns Option
fn find_square_root(x: f64) -> Option<f64> {
    if x < 0.0 {
        None // Negative numbers don't have real square roots
    } else {
        Some(x.sqrt()) // Return square root for non-negative numbers
    }
}

fn main() {
    // Using Result with unwrap_or_else
    let division_result = divide(10.0, 2.0).unwrap_or_else(|err| {
        println!("Error in division: {}", err);
        0.0 // Default value in case of error
    });

    println!("Division result: {}", division_result);

    // Using Option with unwrap_or_else
    let sqrt_result = find_square_root(-9.0).unwrap_or_else(|| {
        println!("Error: Cannot find the square root of a negative number.");
        0.0 // Default value in case of None
    });

    println!("Square root result: {}", sqrt_result);
}