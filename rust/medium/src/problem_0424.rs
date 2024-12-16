struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut counts = [0usize; 26];
        let s_vec = s.chars().collect::<Vec<char>>();

        let mut left_index = 0;
        let mut max_freq = 0;
        let mut res = 0;

        for (right_index, char) in s_vec.iter().enumerate() {
            let char_index = *char as u8 - b'A';
            counts[char_index as usize] += 1;
            max_freq = max_freq.max(counts[char_index as usize]);

            if right_index - left_index + 1 - max_freq > k as usize {
                let char_index = s_vec[left_index] as u8 - b'A';
                counts[char_index as usize] -= 1;
                left_index += 1;
            }

            res = res.max(right_index - left_index + 1);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0424::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::character_replacement("ABBB".to_string(), 2), 4)
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::character_replacement("IMNJJTRMJEGMSOLSCCQICIHLQIOGBJAEHQOCRAJQMBIBATGLJDTBNCPIFRDLRIJHRABBJGQAOLIKRLHDRIGERENNMJSDSSMESSTR".to_string(), 2),
            6
        )
    }
}
