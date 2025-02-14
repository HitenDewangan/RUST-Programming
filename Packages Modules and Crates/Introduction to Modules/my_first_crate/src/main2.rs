// The use keyword

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    use math::{add, subtract};

    let sum: i32 = add(5, 3);
    let difference: i32 = subtract(5, 3);

    println!("Sum: {}, Difference: {}", sum, difference);
}