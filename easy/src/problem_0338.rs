struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut cache = vec![0; n as usize + 1];
        for i in 1..(n + 1) {
            cache[i as usize] = cache[(i >> 1) as usize] + (i & 1);
        }
        cache
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0338::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1])
    }
}
