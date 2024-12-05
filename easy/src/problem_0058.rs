struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut has_seen_char = false;
        let mut len = 0;
        for ch in s.chars().rev() {
            if ch == ' ' {
                if has_seen_char {
                    break;
                }
            } else {
                len += 1;
                has_seen_char = true;
            }
        }

        len
    }
}
