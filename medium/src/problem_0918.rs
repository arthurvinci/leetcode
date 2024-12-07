struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut max, mut total, mut min) = (nums[0], 0, nums[0]);
        let (mut local_max, mut loccal_min) = (0, 0);
        for &num in nums.iter() {
            local_max = local_max.max(0) + num;
            max = max.max(local_max);
            total += num;
            loccal_min = loccal_min.min(0) + num;
            min = min.min(loccal_min);
        }
        if max < 0 {
            max
        } else {
            max.max(total - min)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0918::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_subarray_sum_circular(vec![6, 9, -3]), 15);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::max_subarray_sum_circular(vec![0, 5, 8, -9, 9, -7, 3, -2]),
            16
        );
    }
}
