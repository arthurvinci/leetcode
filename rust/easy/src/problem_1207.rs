use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut arr_map = HashMap::new();

        for num in arr {
            *arr_map.entry(num).or_insert(0) += 1;
        }

        let mut occurences = HashSet::new();
        for occ in arr_map.values() {
            if !occurences.insert(*occ) {
                return false;
            }
        }

        true
    }
}
