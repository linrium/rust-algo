/// Given 2 sequences of strings A and B, determine all of the "universal" characters in A relative to B. Return the list of these universal characters.
/// A string a in A is "universal" relative to B if for each string in B (we will name b) a is a superset of b. This means that a has all of the characters in each b (ignoring order) in quantity at least.
/// ## Example 1:
/// - Input:
///     - A = ["orange", "room", "more"]
///     - B = ["rm", "oo"]
/// - Output: ["room"]
/// - Explanation:
///     - "orange" is missing an "m" so it is not a superset of "rm". It also only has one "o" so it is not a superset of "oo".
///     - "room" is a superset of "rm" and "oo". The is all strings in B so "room" is universal.
///     - "more" is a superset of "rm" since it has an "m" and an "r". It only has one "o" so it is not a superset of "oo".
/// ## Example 2:
/// - Input:
///     - A = ["padding", "css", "randomcs"]
///     - B = ["cs", "c"]
/// - Output: ["css", "randomcs"]
/// - Explanation:
///     - "padding" is missing an "s" so it is not a superset of "cs".
///     - "css" is a superset of "cs".
///     - "randomcs" is a superset of "cs" and "c". It has all of the characters in B in quantity at least.
///
/// ## Constraints:
/// - B.length > 0
///
fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    let mut b_count: [u8; 26] = [0; 26];

    for word in b {
        let word_count = count(&word);
        for i in 0..26 {
            b_count[i] = std::cmp::max(b_count[i], word_count[i]);
        }
    }

    let mut result: Vec<String> = Vec::new();
    for word in a {
        let word_count = count(&word);
        let mut is_universal = true;
        for i in 0..26 {
            if word_count[i] < b_count[i] {
                is_universal = false;
                break;
            }
        }
        if is_universal {
            result.push(word);
        }
    }

    result
}

fn count(s: &String) -> [u8; 26] {
    let mut output: [u8; 26] = [0; 26];
    for c in s.chars() {
        output[c as usize - 'a' as usize] += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_subsets_test() {
        assert_eq!(
            word_subsets(
                vec![
                    "orange".to_string(),
                    "room".to_string(),
                    "more".to_string(),
                ],
                vec!["rm".to_string(), "oo".to_string()],
            ),
            vec!["room".to_string()]
        );
        assert_eq!(
            word_subsets(
                vec!["padding".to_string(), "css".to_string(), "randomcs".to_string()],
                vec!["cs".to_string(), "c".to_string()],
            ),
            vec!["css".to_string(), "randomcs".to_string()]
        );
    }
}