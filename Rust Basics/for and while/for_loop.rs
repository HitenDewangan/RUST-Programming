fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for num in numbers {
        println!("The number is: {}", num);
    }

    // looping over a range
    println!("------------------");
    for num in 1..6 {
        println!("The number is: {}", num);
    }
}