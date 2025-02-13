fn main() {
    for i in 1..=5 {
        
        if i == 3{
            continue; //skips the iteration when i is 3
        }
        println!("The number is: {}", i);
    }
}