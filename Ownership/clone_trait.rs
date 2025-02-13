pub trait Clone {
    fn close(&self) -> Self;
}

fn main() {
    // Using Copy
    let x = 5; // i32 is  Copy
    let y = x; // x is copied to y
    println!("x: {}, y: {}", x, y); // Both x and y are valid

    // Using Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicitly clone s1
    println!("s1: {}, s2: {}", s1, s2); // Both s1 and s2 are valid

}