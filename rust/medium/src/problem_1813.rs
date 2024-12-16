use std::cmp::min;
struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<&str> = sentence1.split(' ').collect();
        let words2: Vec<&str> = sentence2.split(' ').collect();

        let mut longest_prefix = 0;
        for i in 0..min(words1.len(), words2.len()) {
            if words1[i] == words2[i] {
                longest_prefix += 1;
            } else {
                break;
            }
        }

        let mut longest_suffix = 0;
        for i in 0..min(words1.len(), words2.len()) {
            if words1[words1.len() - i - 1] == words2[words2.len() - i - 1] {
                longest_suffix += 1;
            } else {
                break;
            }
        }

        longest_prefix + longest_suffix >= words1.len()
            || longest_prefix + longest_suffix >= words2.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1813::Solution;

    #[test]
    fn test_1() {
        let ok =
            Solution::are_sentences_similar("My name is Haley".to_string(), "My Haley".to_string());
        assert_eq!(ok, true)
    }

    #[test]
    fn test_2() {
        let ok = Solution::are_sentences_similar("of".to_string(), "A lot of words".to_string());
        assert_eq!(ok, false)
    }

    #[test]
    fn test_3() {
        let ok =
            Solution::are_sentences_similar("Eating right now".to_string(), "Eating".to_string());
        assert_eq!(ok, true)
    }

    #[test]
    fn test_4() {
        let ok = Solution::are_sentences_similar(
            "Ogn WtWj HneS".to_string(),
            "Ogn WtWj HneS".to_string(),
        );
        assert_eq!(ok, true);
    }
}
