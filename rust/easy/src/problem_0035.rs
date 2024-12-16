use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len() as i32 - 1;

        while low <= high {
            let mid = low + (high - low) / 2;

            match nums[mid as usize].cmp(&target) {
                Ordering::Less => low = mid + 1,
                Ordering::Equal => return mid,
                Ordering::Greater => high = mid - 1,
            }
        }

        low
    }
}
