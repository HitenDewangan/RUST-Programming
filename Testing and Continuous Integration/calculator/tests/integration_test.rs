use calculator::divide;
mod common;

#[test]
fn test_divide_success() {
    common::setup();
    assert_eq!(divide(10, 2), Ok(5)); // TEst case for successful Division
}

#[test]
fn test_divide_by_zero() {
    common::setup();
    assert_eq!(divide(10, 0), Err(String::from("Cannot divide by zero"))); // Test case for division by zero
}