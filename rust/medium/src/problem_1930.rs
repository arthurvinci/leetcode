struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut first = [-1; 26];
        let mut last = [-1; 26];

        for (index, &b) in bytes.iter().enumerate() {
            let char_i = b as usize - b'a' as usize;
            if first[char_i] == -1 {
                first[char_i] = index as i32
            }
            last[char_i] = index as i32;
        }

        let mut total = 0;

        for char in 0..26 {
            if last[char] != -1 {
                let mut already_used = [false; 26];

                for str_index in (first[char] + 1)..last[char] {
                    let char = bytes[str_index as usize];
                    let char_index = char as usize - b'a' as usize;
                    if !already_used[char_index] {
                        already_used[char_index] = true;
                        total += 1;
                    }
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1930::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::count_palindromic_subsequence("ckafnafqo".to_string()),
            4
        )
    }
}
