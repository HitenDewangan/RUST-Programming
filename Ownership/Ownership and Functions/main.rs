// Ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    // string literal
    let mut s = "hello";
    println!("{}", s);

    s= "world";
    println!("{}", s);

    // String type object
    let mut sobj = String::from("hello");
    println!("{}", sobj);

    sobj.push_str(", world!");
    println!("{}", sobj);

    // Scope and Ownership

    {
        let x = 5; // x is valid from this point
        println!("{}", x);
    } // x is no longer valid
}