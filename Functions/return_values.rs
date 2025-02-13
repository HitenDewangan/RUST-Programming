fn main() {
    let result = add(5, 3);
    println!("The sum is: {}", result);

    let result = subtract(10, 4);
    println!("The difference is: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}