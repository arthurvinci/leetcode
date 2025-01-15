use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut counts = HashMap::new();

        for char in s.chars() {
            let amount = counts.entry(char).or_insert(0);
            *amount += 1;
        }

        let mut new_length = 0;

        for (_, count) in counts {
            if count % 2 == 1 {
                new_length += 1;
            } else {
                new_length += 2;
            }
        }

        new_length
    }
}
