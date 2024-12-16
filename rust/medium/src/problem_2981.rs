use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut letter_count = HashMap::new();
        for char in s.chars() {
            *letter_count.entry(char).or_insert(0) += 1;
        }

        let mut left_index = 0;
        let mut right_index = 0;
        let vec: Vec<char> = s.chars().collect();
        let mut max_special_substring = -1;
        while right_index < vec.len() {
            if vec[left_index] == vec[right_index] {
                if *letter_count.get(&vec[right_index]).unwrap_or(&0) > 2 {
                    if Self::appears_at_least_thrice(
                        &vec[left_index..],
                        &vec[left_index..=right_index],
                    ) {
                        max_special_substring =
                            max_special_substring.max(right_index as i32 + 1 - left_index as i32);
                    }
                    right_index += 1;
                } else {
                    right_index += 1;
                    left_index = right_index;
                }
            } else {
                left_index = right_index;
            }
        }

        max_special_substring
    }

    fn appears_at_least_thrice(word: &[char], substring: &[char]) -> bool {
        let mut count = 0;

        for i in 0..word.len() {
            let mut is_match = true;
            for j in 0..substring.len() {
                if i + j >= word.len() || word[i + j] != substring[j] {
                    is_match = false;
                }
            }
            if is_match {
                count += 1;
            }

            if count > 2 {
                break;
            }
        }
        count > 2
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2981::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::maximum_length("aiii".to_string()), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::maximum_length("aaauuuxxxx".to_string()), 2);
    }
}
