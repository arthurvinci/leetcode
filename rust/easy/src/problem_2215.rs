use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.iter().copied().collect();
        let set2: HashSet<i32> = nums2.iter().copied().collect();

        vec![
            set1.difference(&set2).copied().collect(),
            set2.difference(&set1).copied().collect(),
        ]
    }
}
