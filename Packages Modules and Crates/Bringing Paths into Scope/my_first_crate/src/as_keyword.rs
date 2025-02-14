// as keyword

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
    use crate::math::operations::add as operations_add;
    use crate::math::utils::add as utils_add;

    let sum_operations: i32 = operations_add(5, 3);
    let sum_utils: i32 = utils_add(5, 3);

    println!(
        "Operations Sum: {}, Utils Sum: {}",
        sum_operations, sum_utils
    );
}