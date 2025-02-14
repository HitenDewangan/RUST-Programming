// Relative Paths

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub mod operations {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}

fn main() {
    let sum: i32 = math::add(5, 3); // Relative path within the same file
    let product: i32 = math::operations::multiply(4, 2); // Another relative path
    println!("Sum: {}, Product: {}", sum, product);
}