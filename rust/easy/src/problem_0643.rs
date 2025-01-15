struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut k_sum = 0;
        for i in 0..k as usize {
            k_sum += nums[i];
        }

        let mut max = k_sum as f64 / k as f64;

        let mut left_index = 0;
        for right_index in k as usize..nums.len() {
            k_sum -= nums[left_index];
            k_sum += nums[right_index];
            left_index += 1;
            max = max.max(k_sum as f64 / k as f64);
        }

        max
    }
}
