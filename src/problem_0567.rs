use std::collections::{HashMap};

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let diff = (s2.len() as i64) - (s1.len() as i64);
        if diff < 0{
            false
        }
        else{
            let mut map = HashMap::new();
            for char in s1.chars() {
                map.entry(char).and_modify(|counter| { *counter += 1 }).or_insert(1);
            }
            for i in 0..(diff as usize + 1) {
                let new_s2 = &s2[i..i+s1.len()];
                if Self::is_permutation(map.clone(), &new_s2.to_string())
                {
                    return true
                }
            }
            false
        }
    }

    fn is_permutation(mut original_map: HashMap<char, i32>, to_check: &String) -> bool {

        for char in to_check.chars() {
            match original_map.get(&char) {
                None => return false,
                Some(count) => {
                    if *count == 0{
                        return false;
                    }
                    else{
                        original_map.insert(char, count-1);
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests{
    use crate::problem_0567::Solution;

    #[test]
    fn test_1(){
        let ok = Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string());
        assert_eq!(ok, true)
    }

    #[test]
    fn test_2(){
        let ok = Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string());
        assert_eq!(ok, false)
    }

    #[test]
    fn test_3(){
        let ok = Solution::check_inclusion("adc".to_string(), "dcda".to_string());
        assert_eq!(ok, true)
    }

}

