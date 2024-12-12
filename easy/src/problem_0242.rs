use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars = HashMap::new();
        let mut t_chars = HashMap::new();

        for char in s.chars() {
            *s_chars.entry(char).or_insert(0) += 1;
        }
        for char in t.chars() {
            *t_chars.entry(char).or_insert(0) += 1;
        }

        for (key, value) in s_chars.iter() {
            if *t_chars.get(key).unwrap_or(&-1) != *value {
                return false;
            }
        }

        for (key, value) in t_chars.iter() {
            if *s_chars.get(key).unwrap_or(&-1) != *value {
                return false;
            }
        }

        true
    }
}
