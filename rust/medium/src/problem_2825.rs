struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let str1_vec = str1.as_bytes();
        let str2_vec = str2.as_bytes();

        while j < str2.len() && i < str1.len() {
            if str2_vec[j] % 26 == str1_vec[i] % 26 || str2_vec[j] % 26 == (str1_vec[i] + 1) % 26 {
                j += 1;
            }
            i += 1
        }

        j == str2.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2825::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::can_make_subsequence(
            "abc".to_string(),
            "ad".to_string()
        ))
    }
}
