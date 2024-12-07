use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut banned_set: HashSet<i32> = HashSet::new();
        for ban in banned {
            banned_set.insert(ban);
        }

        let mut sum = 0;
        let mut amount_took = 0;
        for i in 1..=n {
            if !banned_set.contains(&i) {
                sum += i;
                if sum > max_sum {
                    break;
                } else {
                    amount_took += 1;
                }
            }
        }

        amount_took
    }
}
