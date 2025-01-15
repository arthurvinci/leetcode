struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut left_pointer = 0;
        let mut right_pointer = s_chars.len() - 1;

        while left_pointer < right_pointer {
            while !Self::is_vowel(s_chars[left_pointer]) && left_pointer < right_pointer {
                left_pointer += 1;
            }

            while !Self::is_vowel(s_chars[right_pointer]) && left_pointer < right_pointer {
                right_pointer -= 1;
            }

            if Self::is_vowel(s_chars[left_pointer])
                && Self::is_vowel(s_chars[right_pointer])
                && left_pointer < right_pointer
            {
                s_chars.swap(left_pointer, right_pointer);
                left_pointer += 1;
                right_pointer -= 1;
            }
        }

        s_chars.iter().collect()
    }

    fn is_vowel(ch: char) -> bool {
        match ch.to_lowercase().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0345::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_vowels("IceCreAm".to_string()),
            "AceCreIm".to_string()
        )
    }
}
