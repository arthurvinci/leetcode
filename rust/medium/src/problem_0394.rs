struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        let mut repetitions = vec![];
        let mut ret_string = String::new();
        let mut repetition_stack = String::new();

        for char in s.chars() {
            match char {
                '[' => {
                    let stack_index_start = stack.len();
                    let repetition = repetition_stack.parse::<usize>().unwrap();
                    repetition_stack = String::new();
                    repetitions.push((stack_index_start, repetition));
                }
                ']' => {
                    let (stack_index_start, repetition) = repetitions.pop().unwrap();
                    let mut word = stack.split_off(stack_index_start).repeat(repetition);
                    if repetitions.is_empty() {
                        let word: String = word.iter().collect();
                        ret_string.push_str(&word);
                    } else {
                        stack.append(&mut word);
                    }
                }
                c if c.is_numeric() => {
                    repetition_stack.push(c);
                }

                c => {
                    if repetitions.is_empty() {
                        ret_string.push(c);
                    } else {
                        stack.push(c)
                    }
                }
            }
        }

        ret_string
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0394::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_string()),
            "zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef".to_string()
        );
    }
}
