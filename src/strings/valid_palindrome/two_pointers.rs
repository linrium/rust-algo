/// # Problem:
/// Given a string s, determine whether s is a palindrome while only considering alphanumeric characters and ignoring case.
///
/// # Example 1:
/// Input: s = "Race Car"
/// Output: true
/// Explanation: When we ignore case and only consider alphanumeric characters, the string "Race Car" is equivalent to "racecar," which is a palindrome.
///
/// # Approach:
/// - To verify whether the inputted string is a palindrome, we need to make sure that the string reads the same when it is read both forwards and backwards. Furthermore, we have the additional constraint that we need to ignore non-alphanumeric characters and our verification should be case-insensitive. One approach to solve this problem would be to preprocess the string by removing any non-alphanumeric characters and converting all alphabetic characters to a common case (e.g. making all characters lowercase). From here, we would be able to simply use a for-loop and verify that the kth character from the start of the string is equal to the kth character from the end of the string. If this property holds, we would return true; otherwise, we would return false. However, it turns out that we can perform this preprocessing on-the-fly. More precisely, we can do so by keeping two pointers: one pointer starts at the beginning of the string, and the second pointer starts at the end of the string. Next, proceed with a while-loop. In each iteration of the while-loop, we move the leftmost pointer to the next alphanumeric character from the start of the string, and we move the rightmost pointer to the next alphanumeric character from the end of the string. At this point, we know that the characters at the indices of our two pointers are alphanumeric by assumption. Thus, we can convert the characters to a common case and compare the two characters. If the characters are not equal, we can immediately conclude that our string is not a palindrome. Otherwise, we continue processing our string, and we return true only if we can successfully process the entire string.
///
/// # Difficulty: Easy
fn valid_palindrome(s: String) -> bool {
    let mut front = 0;
    let mut back = s.len() - 1;
    while front < back {
        let f = s.chars().nth(front).unwrap();
        let b = s.chars().nth(back).unwrap();
        if !f.is_alphanumeric() {
            front += 1;
        } else if !b.is_alphanumeric() {
            back -= 1;
        } else if f.to_lowercase().to_string() != b.to_lowercase().to_string() {
            return false;
        } else {
            front += 1;
            back -= 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::strings::valid_palindrome::two_pointers::valid_palindrome;

    #[test]
    fn test_valid_palindrome() {
        assert_eq!(valid_palindrome("Race Car".to_string()), true);
    }
}