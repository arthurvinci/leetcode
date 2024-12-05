struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut subsequences = vec![vec![-1; text2.len()]; text1.len()];
        let i = text1.len() - 1;
        let j = text2.len() - 1;

        let text_1_vec: Vec<char> = text1.chars().collect();
        let text_2_vec: Vec<char> = text2.chars().collect();

        Self::internal(&text_1_vec, &text_2_vec, i, j, &mut subsequences);
        subsequences[i][j]
    }

    fn internal(
        text1: &[char],
        text2: &[char],
        i: usize,
        j: usize,
        subsequences: &mut Vec<Vec<i32>>,
    ) {
        let mut result = 0;
        if i > 0 {
            if subsequences[i - 1][j] < 0 {
                Self::internal(text1, text2, i - 1, j, subsequences);
            }
            result = result.max(subsequences[i - 1][j])
        }

        if j > 0 {
            if subsequences[i][j - 1] < 0 {
                Self::internal(text1, text2, i, j - 1, subsequences);
            }
            result = result.max(subsequences[i][j - 1])
        }

        if text1[i] == text2[j] {
            result = result.max(1);
            if i > 0 && j > 0 {
                if subsequences[i - 1][j - 1] < 0 {
                    Self::internal(text1, text2, i - 1, j - 1, subsequences);
                }
                result = result.max(1 + subsequences[i - 1][j - 1]);
            }
        }

        subsequences[i][j] = result;
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1143::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        )
    }
}
