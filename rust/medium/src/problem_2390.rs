struct Solution;
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut char_stack = String::new();

        for ch in s.chars() {
            if ch == '*' {
                char_stack.pop();
            } else {
                char_stack.push(ch);
            }
        }

        char_stack
    }
}
