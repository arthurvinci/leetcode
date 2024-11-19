use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut biggest_sum: i64 = 0;
        let mut k_numbers: HashSet<i32> = HashSet::new();
        let mut current_sum: i64 = 0;
        let mut begin: usize = 0;

        for end in 0..nums.len() {
            if k_numbers.contains(&nums[end]) {
                while nums[begin] != nums[end] {
                    current_sum -= nums[begin] as i64;
                    k_numbers.remove(&nums[begin]);
                    begin += 1;
                }
                begin += 1;
            } else {
                current_sum += nums[end] as i64;
                k_numbers.insert(nums[end]);

                if end - begin + 1 == k as usize {
                    biggest_sum = biggest_sum.max(current_sum);
                    current_sum -= nums[begin] as i64;
                    k_numbers.remove(&nums[begin]);
                    begin += 1;
                }
            }
        }

        biggest_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2461::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3),
            15
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0)
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::maximum_subarray_sum(vec![1, 2, 2], 2), 3)
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::maximum_subarray_sum(vec![9, 9, 9, 1, 2, 3], 3),
            12
        )
    }
}
