fn main() {
    let x = 5;
    let y = x; // copy trait in action
    println!("x: {}, y: {}", x, y); //both x and y are valid

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // this will throw an error
    println!("{}", s2); // s2 is valid

}

// Ownership in Rust is a unique feature that helps in memory management. 
// It is a set of rules that the compiler checks at compile time.

/* A trait defines shared behavior across types, functioning similarly to interfaces in other languages but with greater flexibility and power
 * the copy trait allows types to be duplicated by value instead of being moved
 
 */

// string data stored on heap, stack stores string struct (pointer, length, capacity)

struct string {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

// string struct has three fields: ptr, len, capacity