struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let search_word: Vec<char> = search_word.chars().collect();

        for (index, word) in sentence.split(' ').enumerate() {
            if Self::is_prefix_of_word_internal(word, &search_word) {
                return 1 + index as i32;
            }
        }

        -1
    }

    fn is_prefix_of_word_internal(word: &str, search_word: &[char]) -> bool {
        let chars: Vec<char> = word.chars().collect();

        for (index, char) in search_word.iter().enumerate() {
            if index >= chars.len() || *char != chars[index] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1455::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
            4
        );
    }
}
