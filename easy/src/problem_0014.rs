use std::str::Chars;

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut chars: Vec<Chars> = strs.iter().map(|str| str.chars()).collect();
        let mut prefix = String::new();

        loop {
            let char = match chars[0].next() {
                None => return prefix,
                Some(char) => char,
            };

            for i in 1..chars.len() {
                match chars[i].next() {
                    None => return prefix,
                    Some(ch) => {
                        if char != ch {
                            return prefix;
                        }
                    }
                }
            }

            prefix.push(char);
        }
    }
}
