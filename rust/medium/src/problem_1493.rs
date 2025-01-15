struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut deleted_index = None;
        let mut maximum = 0;

        for (right_index, num) in nums.into_iter().enumerate() {
            if num == 0 {
                if let Some(deleted_index) = deleted_index {
                    left_index = deleted_index + 1;
                }
                deleted_index = Some(right_index);
            }
            maximum = maximum.max(right_index - left_index);
        }

        maximum as i32
    }
}
