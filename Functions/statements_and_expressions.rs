// putting a semicolon at the end of an expression makes it a statement
// and a statement does not return a value

fn main() {
    let y = { // it yields a value, hence is an expression
        let x = 3;
        x + 1 //implicit return, no semicolon
    };

    println!("The value of y is: {}", y); // this is a statement

    // --- if is an expression ---
    let condition = true;
    let number  = if condition { 5 } else { 10 }; // if is an expression

    println!("The value of number is: {}", number); // this is a statement

    // --- match is an expression ---
    let number = 2;

    let message = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "other"
    };

    println!("The message is: {}", message); // this is a statement
}