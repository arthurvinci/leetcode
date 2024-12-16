struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' => {
                    let last_open = stack.pop().unwrap_or('_');
                    if last_open != '(' {
                        return false;
                    }
                }
                ']' => {
                    let last_open = stack.pop().unwrap_or('_');
                    if last_open != '[' {
                        return false;
                    }
                }
                '}' => {
                    let last_open = stack.pop().unwrap_or('_');
                    if last_open != '{' {
                        return false;
                    }
                }

                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}
