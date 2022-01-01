use std::collections::HashSet;

/// # Problem:
/// Given a string, determine the length of the longest possible palindromic string that can be constructed using the characters of the string.
///
/// # Example 1:
/// Input: "aabbc"
/// Output: 5
/// Explanation: The longest palindromes possible are {"abcba", "bacab"}
///
/// # Example 2:
/// Input: "abbcccd"
/// Output: 5
/// Explanation: The original string length is 7, but the longest palindromes are {"cbcbc",  "bcccb"}; 'a' and 'd' were not used.
///
/// # Example 3:
/// Input: "aA"
/// Output: 1
/// Explanation: since the input is case-sensitive; the longest palindromes are {"a", "A"}
///
/// # Example 4:
/// Input: "xyz"
/// Output: 1
///
/// # Example 5:
/// Input: "ccc"
/// Output: 3
///
/// # Hint:
/// 1. We use a set to manually track matches.
///
/// # Associated Companies:
/// - Amazon
/// - Microsoft
/// - Qualcomm
///
/// # Difficulty: Easy
fn longest_palindrome(s: String) -> i32 {
    let mut matching = 0;
    // Create a set for storing the occurrence of the character
    let mut unmatched_chars = HashSet::new();

    // If the character is already present we remove that character from the set
    // And add 1 to the ans as we got an even pair
    // Otherwise we insert that character to the string
    for c in s.chars() {
        if unmatched_chars.contains(&c) {
            unmatched_chars.remove(&c);
            matching += 1;
        } else {
            unmatched_chars.insert(c);
        }
    }

    // The length of even substring
    let longest_palindrome_len = matching * 2;

    // It will constitute the middle element of the string
    if !unmatched_chars.is_empty() {
        // place 1 unmatched odd character in the middle
        longest_palindrome_len + 1
    } else {
        longest_palindrome_len
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::strings::longest_palindrome_construct::with_set::longest_palindrome;

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