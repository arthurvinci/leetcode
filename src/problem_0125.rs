struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: String = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        let ok = cleaned.as_bytes();
        let mut left_pointer = 0;
        let mut right_pointer = cleaned.len() - 1;
        while left_pointer < right_pointer {
            let left_char = ok[left_pointer] as char;
            let right_char = ok[right_pointer] as char;
            if left_char != right_char {
                return false;
            }
            left_pointer += 1;
            right_pointer -= 1;
        }

        true
    }
}
