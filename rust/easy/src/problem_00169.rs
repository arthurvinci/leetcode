struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority_candidate = nums[0];
        let mut count = 1;
        for num in nums.iter().skip(1) {
            if count == 0 {
                majority_candidate = *num;
                count = 1
            } else if *num == majority_candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        majority_candidate
    }
}
