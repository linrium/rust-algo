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
/// - We can use a left and right pointer to advance, as we go we can expand our "window" as the string stays unique and contract it when the property is broken.
///
/// # Complexity:
/// - Time: O(n)
/// - Space: O(n)
///
/// # Companies:
/// - Microsoft
/// - Amazon
///
/// # Difficulty: Medium
fn longest_unique_character_substring(s: String) -> u32 {
    let mut start = 0;
    let mut end = 0;

    let mut mapping = HashMap::new();
    let mut maximum = 0;

    while end < s.len() {
        let c = s.chars().nth(end).unwrap();
        if mapping.contains_key(&c) {
            let v2 = *mapping.get(&c).unwrap();
            start = max(start, v2);
        }
        mapping.insert(c, end + 1);
        maximum = max(maximum, end - start + 1);
        end += 1;
    }

    maximum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_unique_character_substring_test() {
        assert_eq!(5, longest_unique_character_substring("ABCABADEC".to_string()));
        assert_eq!(0, longest_unique_character_substring("".to_string()));
    }
}