fn main() {
    let mut s = String::from("hello");
    let r = &mut s; // Mutable reference to s
    r.push_str(", world!"); // modify the value through the mutable reference

    println!("{}", r); // r is still valid
    println!("{}", s); // s is still valid
}

/*
NOTE : we can only have one mutable reference to a particular piece of data 
at a time.
*/