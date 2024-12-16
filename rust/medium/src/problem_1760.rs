struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut right = *nums.iter().max().unwrap() as usize;
        let mut left = 1usize;

        while left < right {
            let mid = (right + left) / 2;
            let op_needed = nums.iter().fold(0, |acc, x| acc + *x / mid as i32);
            if op_needed > max_operations {
                left = mid;
            } else {
                right = mid;
            }
        }

        right as i32
    }
}
