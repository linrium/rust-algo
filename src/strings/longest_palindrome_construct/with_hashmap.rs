use std::collections::HashMap;

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
    // Create a Map for storing the frequency of the characters
    let mut character_to_count = HashMap::new();

    // Keep storing the frequency of every character in the map
    for c in s.chars() {
        let count = character_to_count.entry(c).or_insert(0);
        *count += 1;
    }

    for (_, count) in character_to_count {
        if count % 2 == 0 {
            matching += count;
        } else {
            matching += count - 1;
        }
    }

    if matching == s.len() {
        matching as i32
    } else {
        matching as i32 + 1
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::strings::longest_palindrome_construct::with_hashmap::longest_palindrome;

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