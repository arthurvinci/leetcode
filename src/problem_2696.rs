struct Solution {}

impl Solution {
    pub fn min_length(s: String) -> i32 {
        if s.is_empty() {
            0
        } else {
            let mut new_s = s.clone();
            loop {
                let s_length = new_s.len();
                new_s = new_s.split("AB").map(String::from).collect();
                new_s = new_s.split("CD").map(String::from).collect();

                if s_length == new_s.len() {
                    break;
                }
            }
            new_s.len() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use crate::problem_2696::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::min_length("ABFCACDB".to_string());
        assert_eq!(ok, 2)
    }

    #[test]
    fn test_2() {
        let ok = Solution::min_length("ACBBD".to_string());
        assert_eq!(ok, 5)
    }
}
