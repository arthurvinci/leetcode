use std::cmp::max;
use std::collections::HashMap;

struct Solution{}


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_encounter: HashMap<char, usize> = HashMap::new();
        let mut longest = 0;
        let mut start_index = 0;
        let mut stop_index = 0;

        for c in s.chars(){
            match last_encounter.insert(c, stop_index){
                None => {}
                Some(past_index) => {
                    if past_index >= start_index {
                        longest = max(stop_index - start_index, longest);
                        start_index = past_index + 1
                    }
                }
            }
            stop_index += 1;
        }

        longest = max(stop_index - start_index, longest);

        longest as i32
    }
}

#[cfg(test)]
mod tests{
    use crate::problem_0003::Solution;

    #[test]
    fn test_1(){
        let ok = Solution::length_of_longest_substring("aab".to_string());
        assert_eq!(ok, 2)
    }

    #[test]
    fn test_2(){
        let ok = Solution::length_of_longest_substring("cdd".to_string());
        assert_eq!(ok, 2)
    }

    #[test]
    fn test_3() {
        let ok = Solution::length_of_longest_substring("dvdf".to_string());
        assert_eq!(ok, 3)
    }

    #[test]
    fn test_4() {
        let ok = Solution::length_of_longest_substring("abba".to_string());
        assert_eq!(ok, 2)
    }
}

