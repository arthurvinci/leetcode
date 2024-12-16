struct Solution;

impl Solution {
    pub fn is_subsequence(t: String, s: String) -> bool {
        let mut t_char = t.chars().peekable();

        if t_char.peek().is_none() {
            return true;
        }

        let mut to_match = t_char.next().unwrap();

        for char in s.chars() {
            if char == to_match {
                if t_char.peek().is_none() {
                    return true;
                } else {
                    to_match = t_char.next().unwrap();
                }
            }
        }

        false
    }
}
