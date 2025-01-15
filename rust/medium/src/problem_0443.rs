struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut last_char = chars[0];
        let mut left_pointer = 0;
        let mut right_pointer = 0;
        let mut repeated = 0;

        while right_pointer < chars.len() {
            if chars[right_pointer] != last_char {
                chars[left_pointer] = last_char;
                left_pointer += 1;
                if repeated > 1 {
                    for c in repeated.to_string().chars() {
                        chars[left_pointer] = c;
                        left_pointer += 1;
                    }
                }
                repeated = 0;
                last_char = chars[right_pointer];
            }

            repeated += 1;
            right_pointer += 1;
        }

        chars[left_pointer] = last_char;
        left_pointer += 1;

        if repeated > 1 {
            for c in repeated.to_string().chars() {
                chars[left_pointer] = c;
                left_pointer += 1;
            }
        }

        while left_pointer < chars.len() {
            chars.pop();
        }

        chars.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0443::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']),
            6
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::compress(&mut vec!['a', 'a', 'a', 'b', 'b', 'a', 'a']),
            6
        );
    }
}
