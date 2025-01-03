struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = vec![0; words.len() + 1];

        for (index, word) in words.iter().enumerate() {
            sum[index + 1] = sum[index];

            if Self::start_and_ends_with_vowel(word) {
                sum[index + 1] += 1;
            }
        }

        queries
            .iter()
            .map(|query| sum[query[1] as usize + 1] - sum[query[0] as usize])
            .collect()
    }

    fn start_and_ends_with_vowel(word: &String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let last = chars.len() - 1;
        (chars[0] == 'a'
            || chars[0] == 'e'
            || chars[0] == 'i'
            || chars[0] == 'o'
            || chars[0] == 'u')
            && (chars[last] == 'a'
                || chars[last] == 'e'
                || chars[last] == 'i'
                || chars[last] == 'o'
                || chars[last] == 'u')
    }
}

#[cfg(test)]
mod test {
    use crate::problem_2559::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::vowel_strings(
                vec![
                    "aba".to_string(),
                    "bcb".to_string(),
                    "ece".to_string(),
                    "aa".to_string(),
                    "e".to_string()
                ],
                vec![vec![0, 2], vec![1, 4], vec![1, 1]]
            ),
            vec![2, 3, 0]
        );
    }
}
