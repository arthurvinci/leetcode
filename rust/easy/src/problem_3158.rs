struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut seen = 0u64;
        let mut sum = 0;
        for num in nums.into_iter() {
            let hash = 1u64 << num as usize;
            if hash & seen > 0 {
                sum ^= num
            }
            seen = seen | hash
        }

        sum
    }
}
#[cfg(test)]
mod test {
    use crate::problem_3158::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1)
    }
}
