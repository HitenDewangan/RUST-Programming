fn main() {
    let s1 = gives_ownership(); // s1 takes the ownership of the returned string
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved to the function, and ownership returned to s3

    // println!("{}", s2); // Error: s2 is no longer valid

    //  ---- Copy Trait : Copying the value instead of moving ownership ----

    let x1 = gives_copy(); // x1 receives the returned integer

    let x2 = 5;
    let x3 = takes_and_returns_copy(x2); // x2 is copied to the function, and a copy is returned to x3

    println!("{}", x2); // x2 is still valid
    println!("{}", x3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // Return the string, transferring ownership to the caller
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { 
    a_string  // Return the string, transferring ownership to the caller
}

// ---- Copy Trait : Copying the value instead of moving ownership ----
fn gives_copy() -> i32 {
    let some_integer = 5;

    some_integer // Return the integer, copying it to the caller
}

fn takes_and_returns_copy(a_integer: i32) -> i32 {
    a_integer // Return the integer, copying it to the caller
}