fn main() {
    greet();
    greet_user("Alice");

    let area = calculate_area(5, 10);
    println!("The area is: {}", area);

    let message1 = add_prefix("ERROR", "File not found");
    let message2 = add_prefix("WARNING", "Low disk space");

    println!("{}", message1);
    println!("{}", message2);
}

fn greet() {
    println!("Hello, world!");
}

fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height // last expression is implicitely returned
}

fn add_prefix(prefix: &str, message: &str) -> String {
    format!("[{}] {}", prefix, message)
}