use std::collections::HashMap;

/// # Problem:
/// Given a list of prefixes prefixes and a sentence sentence, replace all words in sentence that start with any of the given prefixes in prefixes.
///
/// # Example 1:
/// Input:
/// - prefixes = ["cat", "catch", "Alabama"]
/// - sentence = "The cats were catching yarn"
/// Output: "The cat were catch yarn"
/// Explanation: "cats" and "catching" were both replaced by their shortest prefix match "cat"
///
/// # Note:
/// - The shortest matching prefix will win when a word matches multiple prefixes
///
/// # Companies:
/// - Adobe
/// - Amazon
/// - Microsoft
/// - Walmart Labs
///
/// # Complexity:
/// - Time: O(max(m, n^2)) where m is the length of the longest prefix and n is the length of the sentence
/// - Space: O(max(m, n^2)) where m is the length of the longest prefix and n is the length of the sentence
///
/// # Difficulty: Easy
fn replace_words_with_prefix(prefixes: Vec<String>, sentence: String) -> String {
    let mut map: HashMap<String, String> = HashMap::new();
    for prefix in prefixes {
        map.insert(prefix.clone(), prefix);
    }
    let mut words = sentence.split_whitespace().collect::<Vec<_>>();
    for word in words.iter_mut() {
        for j in 0..word.len() {
            let sub = &word[0..j];
            if map.contains_key(sub) {
                *word = sub;
                break;
            }
        }
    }

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_words_with_prefix() {
        assert_eq!(
            replace_words_with_prefix(
                vec!["cat".to_string(), "catch".to_string(), "Alabama".to_string()],
                "The cats were catching yarn".to_string()
            ),
            "The cat were cat yarn"
        );
    }
}