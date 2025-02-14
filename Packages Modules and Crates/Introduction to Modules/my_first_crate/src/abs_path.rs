// Absolute path

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    let sum: i32 = crate::math::add(5, 3); // absolute path
    println!("Sum: {}", sum);
}