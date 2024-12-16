struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut cache = vec![false; s.len() + 1];
        cache[s.len()] = true;

        for index in (0..s.len()).rev() {
            for word in &word_dict {
                if index + word.len() <= s.len() && &s[index..index + word.len()] == word {
                    cache[index] = cache[index + word.len()]
                }
                if cache[index] {
                    break;
                }
            }
        }

        cache[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0139::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ))
    }
}
