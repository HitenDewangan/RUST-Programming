fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1);
    // println!("{}", s1); // s1 is no longer valid
    println!("The length is {}.", len); // s1 is no longer valid

    let s2 = String::from("world");
    let len = calculate_length_borrow(&s2); // Pass a reference to s2
    println!("The length of {} is {}.", s2, len); // s2 is still valid

}

fn calculate_length(s: String) -> usize { // s is a reference to a String
    s.len() // Return the length of the String
}

fn calculate_length_borrow(s: &String) -> usize { // s is a reference to a String
    s.len() // Return the length of the String
}