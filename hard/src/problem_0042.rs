struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_highest = vec![0; height.len()];
        let mut right_highest = vec![0; height.len()];

        left_highest[0] = height[0];
        right_highest[height.len() - 1] = height[height.len() - 1];

        for i in 1..height.len() {
            left_highest[i] = height[i].max(left_highest[i - 1])
        }

        for i in (0..(height.len() - 1)).rev() {
            right_highest[i] = height[i].max(right_highest[i + 1])
        }

        let mut total = 0;
        for i in 0..height.len() {
            total += left_highest[i].min(right_highest[i]) - height[i]
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0042::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9)
    }
}
