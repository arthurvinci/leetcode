struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];

        let mut current_sum = 0;
        for num in nums {
            current_sum = 0.max(current_sum);
            current_sum += num;
            max_sum = max_sum.max(current_sum)
        }

        max_sum
    }
}
