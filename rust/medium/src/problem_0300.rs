struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut subsequence = vec![];
        subsequence.push(nums[0]);

        for num in nums.into_iter().skip(1) {
            if num > *subsequence.last().unwrap() {
                subsequence.push(num)
            } else {
                let mut left_index = 0;
                let mut right_index = subsequence.len() - 1;

                while left_index < right_index {
                    let mid = (left_index + right_index) / 2;

                    if subsequence[mid] < num {
                        left_index = mid + 1;
                    } else {
                        right_index = mid;
                    }
                }
                subsequence[left_index] = num;
            }
        }

        subsequence.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0300::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 1, 7, 4, 6, 2, 7, 5]), 4)
    }
}
