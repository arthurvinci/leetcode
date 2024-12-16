struct Solution;

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut max_beauty = 0;
        let mut left_index = 0;
        let mut mid_index = 0;
        let mut right_index = 0;
        while right_index < nums.len() {
            while mid_index < nums.len() - 1 && nums[mid_index + 1] - nums[left_index] <= 2 * k {
                mid_index += 1;
            }
            let mid_value = nums[mid_index] - 0.max(nums[mid_index] - nums[left_index] - k);

            right_index = mid_index;

            while right_index < nums.len() && nums[right_index] - mid_value <= k {
                max_beauty = max_beauty.max(right_index + 1 - left_index);
                right_index += 1;
            }

            left_index += 1;
        }

        max_beauty as i32
    }
}

#[cfg(test)]
mod test {
    use crate::problem_2779::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3)
    }
}
