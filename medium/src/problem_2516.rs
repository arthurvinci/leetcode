struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;

        s.chars()
            .for_each(|x| Self::count(x, &mut a, &mut b, &mut c, 1));
        let s_chars: Vec<char> = s.chars().collect();

        if a < k || b < k || c < k {
            -1
        } else {
            let mut a_outside = a;
            let mut b_outside = b;
            let mut c_outside = c;

            let mut max_window = 0;
            let mut left_pointer = 0;

            for (right_pointer, ch) in s_chars.iter().enumerate() {
                Self::count(*ch, &mut a_outside, &mut b_outside, &mut c_outside, -1);

                while left_pointer <= right_pointer
                    && (c_outside < k || b_outside < k || c_outside < k)
                {
                    Self::count(
                        s_chars[left_pointer],
                        &mut a_outside,
                        &mut b_outside,
                        &mut c_outside,
                        1,
                    );
                    left_pointer += 1
                }
                if left_pointer <= right_pointer
                    && a_outside >= k
                    && b_outside >= k
                    && c_outside >= k
                {
                    max_window = max_window.max(right_pointer + 1 - left_pointer)
                }
            }

            s.len() as i32 - max_window as i32
        }
    }

    fn count(ch: char, a_count: &mut i32, b_count: &mut i32, c_count: &mut i32, to_add: i32) {
        match ch {
            'a' => *a_count += to_add,
            'b' => *b_count += to_add,
            'c' => *c_count += to_add,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2516::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
    }
}
