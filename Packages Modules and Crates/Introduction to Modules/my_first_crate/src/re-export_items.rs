// Re-exporting Items with pub use

mod math {
    pub mod operations {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }

    pub use self::operations::multiply; // self refers to the current module
}

fn main() {
    let product: i32 = math::multiply(4, 2); // Now accessible directly from math
    println!("Product: {}", product);
}