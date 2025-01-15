struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut corrected_index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(corrected_index, i);
                corrected_index += 1;
            }
        }
    }
}
