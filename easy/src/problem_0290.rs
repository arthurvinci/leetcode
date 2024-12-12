use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<String> = s.split(' ').map(|x| x.to_string()).collect();
        if pattern.len() != words.len() {
            false
        } else {
            let mut patterned = HashMap::new();
            let mut already_used = HashMap::new();

            for (index, char) in pattern.chars().enumerate() {
                let mapped_word = patterned.entry(char).or_insert(words[index].clone());
                let mapped_char = already_used.entry(words[index].clone()).or_insert(char);
                if *mapped_word != words[index] || *mapped_char != char {
                    return false;
                }
            }
            true
        }
    }
}
