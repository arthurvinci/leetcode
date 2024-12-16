use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut elements = BinaryHeap::from(nums);

        while elements.len() > n + 1 - k as usize {
            elements.pop();
        }

        *elements.peek().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0215::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 4, 5, 6], 2), 5);
    }
}
