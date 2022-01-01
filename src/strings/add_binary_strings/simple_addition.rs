/// # Problem:
/// Given two strings s1 and s2 consisting of only ones and zeros, return a string representing the sum of s1 and s2 when they are considered as binary numbers.
///
/// # Example 1:
/// Input: s1 = "101", s2 = "1"
/// Output: "110"
/// Explanation: In base 10, “101” represents 5, and “1” represents 1. Their sum is 6, which is “110” in binary.
///
/// # Note:
/// - Neither s1 nor s2 have any leading zeros. The returned string should not have any leading zeros either.
///
/// # Approach:
/// - We can simulate the process typically used to carry out base-10 addition with some minor changes due to the fact that we are working in binary. As per usual, we process our two numbers in reverse order. At each step, we compute the sum of the two binary digits in the same column and the digit-wise sum modulo two is added to the front of our answer. While performing these addition operations on the digits of our binary strings, we must maintain a "carry" variable representing whether or not our sum has exceeded one at any given point. This carry variable is maintained and affects our subsequent sum digit-wise sum computations. Note that this is almost identical to how one would carry out base-10 addition on paper. The key difference is that we are taking our digit-wise addition operations modulo 2 rather than modulo 10.
///
/// # Companies:
/// - Microsoft
/// - Facebook
/// - Amazon
///
/// # Complexity:
/// - Time: ?
/// - Space: ?
///
/// # Difficulty: Easy
fn add_binary_strings(s1: String, s2: String) -> String {
    let mut i = (s1.len() - 1) as i32;
    let mut j = (s2.len() - 1) as i32;
    let mut carry = 0;
    let mut result = String::new();
    while i >= 0 || j >= 0 {
        let mut sum = carry;

        if i >= 0 {
            sum += s1.chars().nth(i as usize).unwrap() as u8 - b'0';
            i -= 1;
        }

        if j >= 0 {
            sum += s2.chars().nth(j as usize).unwrap() as u8 - b'0';
            j -= 1;
        }

        result.push(if sum % 2 == 0 { '0' } else { '1' });
        carry = sum / 2;
    }

    if carry == 1 {
        result.push('1');
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_add_binary_strings() {
        let s1 = "101".to_string();
        let s2 = "1".to_string();
        let result = super::add_binary_strings(s1, s2);
        let expected = "110".to_string();
        assert_eq!(result, expected);
    }
}