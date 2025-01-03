struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold([0; 32], |mut acc, &num| {
                for i in 0..32 {
                    if num & (1 << i) != 0 {
                        acc[i] += 1
                    };
                }
                acc
            })
            .into_iter()
            .rev()
            .fold(0, |acc, bit| (acc * 2) + bit % 3)
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0137::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3)
    }
}
