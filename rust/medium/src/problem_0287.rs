struct Solution;

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;

            if nums[index] < 0 {
                return nums[i].abs();
            } else {
                nums[index] *= -1;
            }
        }
        -1
    }
}
