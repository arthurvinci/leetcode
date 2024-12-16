use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_index = 0;
        let mut right_index = nums.len();

        while left_index < right_index {
            let mid = (left_index + right_index) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => {
                    left_index = mid + 1;
                }
                Ordering::Equal => {
                    left_index = mid;
                    break;
                }
                Ordering::Greater => {
                    right_index = mid;
                }
            }
        }

        if nums[left_index] == target {
            left_index as i32
        } else {
            -1
        }
    }
}
