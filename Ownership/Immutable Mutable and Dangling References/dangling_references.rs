/*
 * A dangling reference is a reference that points to invalid or deallocated memory
 
 * Rust's ownership system and borrow checker prevent dangling references at compile time
*/

fn main() {
    let r;
    {
        let x = 5;
        r = &x; // r points to x
    }
    // x goes out of scope and is deallocated
    // r still points to x, but x is no longer valid
    println!("{}", r); // Error: r points to deallocated memory
}