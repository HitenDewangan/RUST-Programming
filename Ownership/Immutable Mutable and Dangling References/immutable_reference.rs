// Immutable References with Stack-Allocated Data

fn main() {
    let x = 5;

    let r1 = &x; // Immutable reference to x
    let r2 = &x; // Another immutable reference to x

    println!("r1: {}, r2: {}", r1, r2); // Both references are valid and can be used
}

// Immutable References with Heap-Allocated Data

// fn main() {
//     let s = String::from("hello");

//     let r1 = &s; // Immutable reference to s
//     let r2 = &s; // Another immutable reference to s

//     println!("r1: {}, r2: {}", r1, r2); // Both references are valid and can be used
// }