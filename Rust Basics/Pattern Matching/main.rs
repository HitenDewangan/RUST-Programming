fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    println!("------------------");

    let number = 4;

    match number {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        _ => println!("The number is something else"),
    }

    // OR (|) operator
    println!("------------------");
    let number = 2;

    match number {
        1 | 2 | 3 => println!("One, two or three"),
        _ => println!("Something else"),
    }

    //MAtching ranges
    println!("------------------");
    let number = 7;

    match number {
        1..=5 => println!("Between one and five"),
        6..=10 => println!("Between six and ten"),
        _ => println!("Something else"),
    }

    // Destructuring with "match"
    let name = Some("Alice");
    let age = Some(30);

    match (name, age) {
        (Some(name), Some(age)) => {
            println!("Name: {}, Age: {}", name, age);
        },
        (Some(name), None) => {
            println!("Name: {}, Age: Unknown", name);
        },
        (None, Some(age)) => {
            println!("Name: Unknown, Age: {}", age);
        },
        (None, None) => {
            println!("Name: Unknown, Age: Unknown");
        },
    }

    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("The numbers are equal"),
        (x, y) if x + y == 0 => println!("The numbers are opposites"),
        _ => println!(" special properties"),
    }

    // ignoring values in Patterns
    println!("------------------");
    let pair = (2, 8);

    match pair {
        (x, _) => println!("The first element is: {}", x),
    }
}