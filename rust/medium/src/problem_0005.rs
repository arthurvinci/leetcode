struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = 0;

        fn expand(s: &Vec<char>, mut i: isize, mut j: isize, left: &mut usize, right: &mut usize) {
            while i >= 0 && j < s.len() as isize && s[i as usize] == s[j as usize] {
                if (j - i) as usize > *right - *left {
                    *left = i as usize;
                    *right = j as usize;
                }
                i -= 1;
                j += 1;
            }
        }

        for i in 0..s.len() {
            expand(&s_chars, i as isize, i as isize, &mut left, &mut right);

            expand(&s_chars, i as isize, i as isize + 1, &mut left, &mut right);
        }

        s_chars[left..=right].iter().collect()
    }
}
