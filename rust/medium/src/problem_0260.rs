struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, &x| acc ^ x);
        let lower_order_bit = xor & -xor;
        let mut number1 = 0;
        let mut number2 = 0;

        for num in nums {
            if num & lower_order_bit == 0 {
                number1 ^= num
            } else {
                number2 ^= num
            }
        }

        vec![number1, number2]
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0260::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::single_number(vec![7, 2, 7, 3, 2, 5]), vec![3, 5])
    }
}
