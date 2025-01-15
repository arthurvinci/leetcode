struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left_index = 0;
        let mut flipped = 0;
        let mut maximum = 0;

        for (right_index, num) in nums.iter().enumerate() {
            if *num == 0 {
                flipped += 1;
            }

            if flipped == k + 1 {
                while nums[left_index] == 1 && left_index < right_index {
                    left_index += 1;
                }
                left_index += 1;
                flipped -= 1;
            }

            maximum = maximum.max(right_index - left_index + 1)
        }

        maximum as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1004::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        )
    }
}
