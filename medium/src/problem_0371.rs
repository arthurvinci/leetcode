struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let tmp_a = a;
            a ^= b;
            b = (tmp_a & b) << 1;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0371::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_sum(-9, 1), -8)
    }
}
