use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut sorted: Vec<(i32, usize)> = nums
            .into_iter()
            .enumerate()
            .map(|(i, val)| (val, i))
            .collect();
        sorted.sort();
        sorted.reverse();

        let mut marked = HashSet::new();
        let mut score = 0;
        while !sorted.is_empty() {
            let element = sorted.pop().unwrap();
            if marked.insert(element.1 as i32) {
                score += element.0 as i64;
                marked.insert(element.1 as i32 + 1);
                marked.insert(element.1 as i32 - 1);
            }
        }

        score
    }
}
