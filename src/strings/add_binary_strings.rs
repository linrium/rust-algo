// Given two strings s1 and s2 consisting of only ones and zeros, return a string representing the sum of s1 and s2 when they are considered as binary numbers.
fn add_binary_strings(s1: String, s2: String) -> String {
    let mut i = (s1.len() - 1) as i32;
    let mut j = (s2.len() - 1) as i32;
    let mut carry = 0;
    let mut result = String::new();
    while i >= 0 || j >= 0 {
        let mut sum = carry;

        if i >= 0 {
            sum += s1.chars().nth(i as usize).unwrap() as u32 - '0' as u32;
            i -= 1;
        }

        if j >= 0 {
            sum += s2.chars().nth(j as usize).unwrap() as u32 - '0' as u32;
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