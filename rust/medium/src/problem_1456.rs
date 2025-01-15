struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut left_index = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut maximum = 0;

        for i in 0..(k as usize) {
            if Self::is_vowel(chars[i]) {
                maximum += 1;
            }
        }

        let mut current_vowel_count = maximum;

        for right_index in (k as usize)..s.len() {
            if Self::is_vowel(chars[left_index]) {
                current_vowel_count -= 1;
            }

            if Self::is_vowel(chars[right_index]) {
                current_vowel_count += 1;
            }

            left_index += 1;

            maximum = maximum.max(current_vowel_count);
        }

        maximum
    }

    fn is_vowel(ch: char) -> bool {
        match ch.to_lowercase().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}
