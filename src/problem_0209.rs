struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut minimum_length = nums.len() as i32 + 1;
        let mut l_pointer = 0;
        let mut current_sum = 0;

        for r_pointer in 0..nums.len() {
            current_sum += nums[r_pointer];

            while current_sum >= target && l_pointer <= r_pointer{
                minimum_length = minimum_length.min(1+(r_pointer - l_pointer) as i32);
                current_sum -= nums[l_pointer];
                l_pointer += 1;
            }
        }


        if minimum_length == nums.len() as i32 + 1 {
            0
        }
        else {
            minimum_length
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0209::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1,2,3,4,5]),
            3
        )
    }
}