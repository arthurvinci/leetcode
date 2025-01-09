struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut current_sum = 0;
        let mut smallest_sum = 0;

        for num in nums {
            current_sum += num;
            smallest_sum = smallest_sum.min(current_sum);
        }

        let value = 1 - smallest_sum;
        value.max(1)
    }
}
