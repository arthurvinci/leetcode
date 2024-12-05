struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            nums[0]
        } else {
            let mut n_1 = nums[0];
            let mut n_2 = nums[1].max(nums[0]);

            for item in nums.iter().skip(2) {
                let tmp = n_2;
                n_2 = n_2.max(n_1 + *item);
                n_1 = tmp;
            }
            n_2
        }
    }
}
