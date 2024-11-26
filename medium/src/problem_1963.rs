struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut counter = 0;

        for char in s.chars() {
            if char == '[' {
                counter += 1
            } else if counter > 0 {
                counter -= 1;
            }
        }

        (counter + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1963::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_swaps("][][".to_string()), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
    }
}
