use std::cmp::max;
use std::collections::HashMap;

/// # Problem:
/// Given a string s, return the length of the longest substring of s without repeating characters.
///
/// # Example 1:
/// Input: s = "ABCABADEC"
/// Output: 5
/// Explanation: Though there are substrings such as "AB" and "ABC" that have all unique characters, "BADEC" is the longest unique character substring.
///
/// # Example 2:
/// Input: s = ""
/// Output: 0
///
/// # Constraints:
/// - All letters will be uppercase A-Z
///
/// # Approach:
/// - We can check all substrings and check if each substring has unique characters. We return the length of the longest unique character substring over all substrings.
///
/// # Complexity:
/// - Time: O(n^3)
/// - Space: O(n)
///
/// # Companies:
/// - Microsoft
/// - Amazon
///
/// # Difficulty: Medium
fn longest_unique_character_substring(s: String) -> u32 {
    let mut maximum = 0;

    for i in 0..s.len() {
        for j in i..s.len() {
            let mut mapping = HashMap::new();
            let mut unique = true;

            for k in i..=j {
                if mapping.contains_key(&s[k..k + 1]) {
                    unique = false;
                    break;
                }

                mapping.insert(s[k..k + 1].to_string(), true);
            }

            if unique {
                maximum = max(maximum, j - i + 1);
            }
        }
    }

    maximum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_unique_character_substring_test() {
        assert_eq!(
            5,
            longest_unique_character_substring("ABCABADEC".to_string())
        );
        assert_eq!(0, longest_unique_character_substring("".to_string()));
    }
}
