use std::collections::{HashSet};

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut seen = HashSet::with_capacity(k + 1);

        for (i, &num) in nums.iter().enumerate() {
            if !seen.insert(num) {
                return true;
            }
            if i + 1 > k {
                seen.remove(&nums[i - k]);
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::problem_0219::Solution;

    #[test]
    fn test_1() {
        assert!(
            !Solution::contains_nearby_duplicate(vec![1,2,1], 0)
        )
    }
}