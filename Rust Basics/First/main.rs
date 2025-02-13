const SERVER_NAME: &str = "localhost";

macro_rules! greet {
    () => {
        println!("Hello, Rustaceans!");
    };
}

macro_rules! greet_with_name {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

macro_rules! log {
    ($msg:expr) => {
        println!("[LOG]: {}", $msg);
    };
}

fn main() {
    // this is a comment
    println!("Hello, world!");
    
    let y = 10;
    println!("The value of y is: {}", y);
    // y = 20; // this line will cause a compilation error
    println!("The value of y is: {}", y);
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);
    
    println!("Server name: {}", SERVER_NAME);
    
    greet!();
    greet_with_name!("Alice");
    
    log!("Application started");

    let number = 11;
    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is equal to 10");
    } else {
        println!("The number is greater than 10");
    }

    let number = 25;
    if number % 5 == 0 && number % 2 == 0 {
        println!("The number is divisible by 5 and 2");
    } else if number % 5 == 0 {
        println!("The number is divisible by 5");
    } else {
        println!("The number is not divisible by 5");
    }

    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("The value of number is: {}", number);

    loop{
        println!("This is an infinite loop");
        break;
    }

    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {}", count);
        if count == 5 {
            break;
        }
    }
    println!("Loop ended at count: {}", count);

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 { //returning a value from the loop
            break count * 2;
        }
    };

    println!("The Result is: {}", result);

}