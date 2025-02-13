fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // Slice of arr from index 1 to 4 (exclusive)
    println!("{:?}", slice); // [2, 3, 4]   
}