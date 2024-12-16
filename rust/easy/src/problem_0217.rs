use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();

        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0217::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert!(ok)
    }

    #[test]
    fn test_2() {
        let ok = Solution::contains_duplicate(vec![1, 2, 3, 4]);
        assert!(!ok)
    }

    #[test]
    fn test_3() {
        let ok = Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert!(ok)
    }
}
