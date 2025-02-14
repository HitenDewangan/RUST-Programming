pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn is_palindrome(input: &str) -> bool {
    let cleaned_input: String = input.chars()
                                    .filter(|c| c.is_alphanumeric())
                                    .collect::<String>()
                                    .to_lowercase();
    cleaned_input == reverse(&cleaned_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("wizard"), "draziw");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("A man, a plan, a canal, Panama"));
        assert!(!is_palindrome("Rustacean"));
    }
}