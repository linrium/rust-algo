/// # Problem:
/// Given a string s, determine whether s is a palindrome while only considering alphanumeric characters and ignoring case.
///
/// # Example 1:
/// Input: s = "Race Car"
/// Output: true
/// Explanation: When we ignore case and only consider alphanumeric characters, the string "Race Car" is equivalent to "racecar," which is a palindrome.
///
/// # Difficulty: Easy
fn valid_palindrome(s: String) -> bool {
    let mut chars = s.chars().filter(|c| c.is_alphanumeric());
    let mut rev_chars = chars.clone().rev();
    while let Some(c) = chars.next() {
        if c.to_lowercase().to_string() != rev_chars.next().unwrap().to_lowercase().to_string() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::strings::valid_palindrome::reverse_words::valid_palindrome;

    #[test]
    fn test_valid_palindrome() {
        assert_eq!(valid_palindrome("Race Car".to_string()), true);
    }
}