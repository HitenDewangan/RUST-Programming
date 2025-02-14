mod math {
    pub mod operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }

    pub mod utils {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b + 1 // Different implementation
        }
    }
}

fn main() {

    // nested paths
    use crate::math::operations::{self, add};

    // glob opreator
    use crate::math::operations::*;
    let sum_operations: i32 = add(5, 3);

    println!("Operations Sum: {}", sum_operations);
}