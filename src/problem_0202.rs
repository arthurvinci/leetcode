use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut already_visited = HashSet::new();
        let mut computed_value = n as u32;
        loop {
            if computed_value == 1 {
                return true;
            }

            if !already_visited.insert(computed_value) {
                return false;
            }
            computed_value = Self::compute_value(computed_value);
        }
    }

    fn compute_value(n: u32) -> u32 {
        let ns: Vec<char> = n.to_string().chars().collect();
        let mut sum = 0;
        for ch in ns {
            let nb = ch.to_digit(10).unwrap();
            sum += nb * nb
        }
        sum
    }
}
