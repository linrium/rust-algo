use std::collections::HashMap;

// Given a set of strings words and a string pattern return a list of all of the strings in words that matches the pattern of the pattern string.
fn find_and_replace_pattern(words: Vec<String>, pattern: &String) -> Vec<String> {
    let mut matches = vec![];
    let hash = encode_string(pattern);

    for word in words {
        let encoded = encode_string(&word);
        if encoded == hash && word.len() == pattern.len() {
            matches.push(word);
        }
    }

    matches
}

fn encode_string(text: &String) -> String {
    let mut map = HashMap::new();
    let mut res = "".to_string();
    let mut i = 0;

    for c in text.chars() {
        if !map.contains_key(&c) {
            map.insert(c, i);
            i += 1;
        }
        let ch = match map.get(&c) {
            Some(x) => x.to_string(),
            None => "0".to_string(),
        };

        res += ch.as_str();
    }

    res
}

#[cfg(test)]
mod tests {
    // Test find_and_replace_pattern
    use super::*;

    #[test]
    fn test_find_and_replace_pattern_1() {
        let words = vec!["aa".to_string(), "bb".to_string()];
        let pattern = "cc".to_string();
        let expected = vec!["aa".to_string(), "bb".to_string()];
        assert_eq!(find_and_replace_pattern(words, &pattern), expected);
    }

    #[test]
    fn test_find_and_replace_pattern_2() {
        let words = vec!["aac".to_string(), "bbc".to_string(), "bcb".to_string(), "yzy".to_string()];
        let pattern = "ghg".to_string();
        let expected = vec!["bcb".to_string(), "yzy".to_string()];
        assert_eq!(find_and_replace_pattern(words, &pattern), expected);
    }

    #[test]
    fn test_find_and_replace_pattern_3() {
        let words = vec!["aa".to_string(), "bb".to_string()];
        let pattern = "zy".to_string();
        let expected = vec![] as Vec<String>;
        assert_eq!(find_and_replace_pattern(words, &pattern), expected);
    }

}