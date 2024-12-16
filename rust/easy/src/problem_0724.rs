struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sums = vec![nums[0]];

        for num in nums.iter().skip(1) {
            sums.push(*num + *sums.last().unwrap());
        }

        for i in 0..sums.len() {
            let mut left_sum = 0;
            let mut right_sum = 0;

            if i > 0 {
                left_sum = sums[i - 1]
            }

            if i < sums.len() - 1 {
                right_sum = sums[sums.len() - 1] - sums[i];
            }

            if left_sum == right_sum {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0724::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }
}
