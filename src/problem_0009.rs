struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let to_string = x.to_string();
        let chars: Vec<char> = to_string.chars().collect();

        let mut left_index = 0;
        let mut right_index = chars.len() - 1;

        while left_index < right_index {
            if chars[left_index] != chars[right_index] {
                return false;
            }

            left_index += 1;
            right_index -= 1;
        }
        true
    }
}
