use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_to_t_map = HashMap::new();
        let mut t_to_s_map = HashMap::new();
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();

        for index in 0..s.len() {
            let s_img = s_to_t_map.entry(s_vec[index]).or_insert(t_vec[index]);
            let t_img = t_to_s_map.entry(t_vec[index]).or_insert(s_vec[index]);
            if *s_img != t_vec[index] || *t_img != s_vec[index] {
                return false;
            }
        }

        true
    }
}
