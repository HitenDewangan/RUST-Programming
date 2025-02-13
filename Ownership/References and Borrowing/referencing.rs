/*
Immutable References in Rust(&T)
---------------------------------
An immutable reference allows you to borrow a value without taking ownership and without modifying it.
*/

fn main() {
    // let reference = &value; // Create an immutable reference to value
    let x = 10;
    let y = &x; // Can read, but not modify x through y
    println!("The value of x is: {}", y);
}