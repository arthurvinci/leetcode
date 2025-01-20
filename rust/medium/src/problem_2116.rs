struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let (mut lo, mut hi) = (0i32, 0i32);
        for (b1, b2) in s.bytes().zip(locked.bytes()) {
            match (b1, b2) {
                (b'(', b'1') => {
                    lo += 1;
                    hi += 1;
                }
                (b')', b'1') => {
                    if hi == 0 {
                        return false;
                    }
                    lo = (lo - 1).abs();
                    hi -= 1;
                }
                (_, b'0') => {
                    lo = (lo - 1).abs();
                    hi += 1;
                }
                _ => unreachable!(),
            }
        }
        lo == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2116::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::can_be_valid(
            "))()))".to_string(),
            "010100".to_string()
        ))
    }
}
