fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // Slice of arr from index 1 to 4 (exclusive)
    println!("{:?}", slice); // [2, 3, 4]   

    let s = String::from("hello, world");
    let hello = &s[0..5]; // Slice from index 0 to 5 (exclusive)
    let world = &s[7..12]; // Slice from index 7 to 12 (exclusive)
    println!("{}, {}", hello, world); // hello, world


    // --- mutable slices ---
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..4]; // Mutable slice of arr from index 1 to 4 (exclusive)
    slice[0] = 10; // modify the first element of the slice
    println!("{:?}", arr); // [10, 3, 4]
}