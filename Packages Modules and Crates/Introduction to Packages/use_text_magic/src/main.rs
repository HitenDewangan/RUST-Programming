fn main() {
    let reversed: String = text_magic::reverse("Rust");
    println!("Reversed: {}", reversed);

    let is_palindrome: bool = text_magic::is_palindrome("A man, a plan, a canal, Panama");
    println!("Is palindrome: {}", is_palindrome);
}