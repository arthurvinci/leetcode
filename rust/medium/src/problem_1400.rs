use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            false
        } else {
            let mut char_count = HashMap::new();
            for char in s.chars() {
                let amount = char_count.entry(char).or_insert(0);
                *amount += 1;
            }

            let mut odd_count = 0;
            for (_, count) in char_count {
                if count % 2 != 0 {
                    odd_count += 1;
                }
            }

            odd_count <= k as usize
        }
    }
}
