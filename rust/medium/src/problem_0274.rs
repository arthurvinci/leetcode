use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let mut left_index = 0;
        let mut right_index = citations.len() - 1;

        while left_index < right_index {
            let mid = (left_index + right_index) / 2;
            match citations[mid].cmp(&(1 + mid as i32)) {
                Ordering::Less => {
                    right_index = mid;
                }
                Ordering::Equal => {
                    left_index = mid;
                    break;
                }
                Ordering::Greater => {
                    left_index = mid + 1;
                }
            }
        }

        citations[left_index]
    }
}
