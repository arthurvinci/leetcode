struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let n = s.len();
        let mut left_zeros = vec![0; n];
        let mut right_ones = vec![0; n];

        let s_chars: Vec<char> = s.chars().collect();

        for i in 0..n {
            if i > 0 {
                left_zeros[i] = left_zeros[i - 1];
                right_ones[n - 1 - i] = right_ones[n - 1 - i + 1];
            }

            if s_chars[i] == '0' {
                left_zeros[i] += 1;
            }

            if s_chars[n - 1 - i] == '1' {
                right_ones[n - 1 - i] += 1;
            }
        }

        let mut max = 0;

        for i in 1..n {
            max = max.max(right_ones[i] + left_zeros[i])
        }

        max
    }
}

#[cfg(test)]
mod test {
    use crate::problem_1422::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_score("011101".to_string()), 5);
    }
}
