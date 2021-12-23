use std::collections::HashSet;

// Given a string, determine the length of the longest possible palindromic string that can be constructed using the characters of the string.
fn longest_palindrome(s: String) -> i32 {
    let mut matching = 0;
    let mut unmatched_chars = HashSet::new();

    for c in s.chars() {
        if unmatched_chars.contains(&c) {
            unmatched_chars.remove(&c);
            matching += 1;
        } else {
            unmatched_chars.insert(c);
        }
    }

    let longest_palindrome_len = matching * 2;

    if !unmatched_chars.is_empty() {
        longest_palindrome_len + 1
    } else {
        longest_palindrome_len
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::strings::longest_palindrome_construct::longest_palindrome;

    // Test longest_palindrome
    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("aabbc".to_string()), 5);
        assert_eq!(longest_palindrome("abbcccd".to_string()), 5);
        assert_eq!(longest_palindrome("aA".to_string()), 1);
        assert_eq!(longest_palindrome("xyz".to_string()), 1);
        assert_eq!(longest_palindrome("ccc".to_string()), 3);
    }
}