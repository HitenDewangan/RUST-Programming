// fn main() {
//     println!("Hello, world!");
// }

use regex::Regex;

fn main() {
    let email: &str = "example@example.com";
    let email_pattern: &str = r"^[\w._%+-]+@[\w.-]+\.[a-zA-Z]{2,}$";
    let re: Regex = Regex::new(email_pattern).unwrap();

    if re.is_match(email) {
        println!("'{}' is a valid email address.", email);
    } else {
        println!("'{}' is not a valid email address.", email);
    }
}