struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut result: u32 = x & 1;

        for _ in 1..32{
            result = result << 1;
            x = x >> 1;
            if x & 1 == 1{
                result+=1
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0190::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192)
    }
}