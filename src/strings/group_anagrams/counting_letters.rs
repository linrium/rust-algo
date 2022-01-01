use std::collections::HashMap;

/// Group Anagrams
/// An anagram is a sequence of characters formed by rearranging the letters of another full sequence of characters.
/// Given a list of words words, return a list of lists with each word in words grouped with its other anagrams.
///
/// For each word, we can take letter counts and use that as a signature to track anagram groups.
///
/// # Example 1:
/// Input: ["eat", "bat", "ate", "tab", "tea", "eat"]
/// Output:
/// [
///   ["eat", "ate", "tea", "eat"],
///   ["bat", "tab"]
/// ]
/// Explanation:
/// 1.) "eat", "ate", "tea", "eat" are all anagrams of the same "signature".
/// 2.) "bat", "tab" are anagrams with respect to each other as well.
///
/// # Constraints:
/// - No string in words will be null or empty ("")
///
/// # Complexity:
/// - Time: O(n * k), where n is the number of words and k is the average length of words.
/// - Space: O(n * k), where n is the number of words and k is the average length of words.
fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for word in words {
        let mut letters = [0; 26];
        for c in word.chars() {
            letters[(c as u8 - b'a') as usize] += 1;
        }

        let key = letters
            .iter()
            .fold(String::new(), |acc, &x| acc + &x.to_string());
        let value = map.entry(key).or_insert(Vec::new());
        value.push(word);
    }

    map.into_iter().map(|(_, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let mut actual = group_anagrams(vec![
            "eat".to_string(),
            "bat".to_string(),
            "ate".to_string(),
            "tab".to_string(),
            "tea".to_string(),
            "eat".to_string(),
        ]);
        actual.sort();

        assert_eq!(
            actual,
            vec![
                vec!["bat".to_string(), "tab".to_string()],
                vec![
                    "eat".to_string(),
                    "ate".to_string(),
                    "tea".to_string(),
                    "eat".to_string()
                ],
            ]
        );
    }
}
