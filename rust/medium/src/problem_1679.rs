struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnts = HashMap::new();
        let mut res = 0;
        for num in nums {
            match cnts.get_mut(&(k - num)) {
                Some(&mut 0) | None => {
                    *cnts.entry(num).or_insert(0) += 1;
                }
                Some(cnt) => {
                    *cnt -= 1;
                    res += 1;
                }
            }
        }
        res
    }
}
