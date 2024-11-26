use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut right_max_index = vec![n - 1; n];
        let mut left_max_index = vec![0; n];

        // Storing considered indices
        let mut current_stack: Vec<usize> = vec![];
        for (i, height) in heights.iter().enumerate() {
            while !current_stack.is_empty() {
                let last_element = *current_stack.last().unwrap();
                if *heights.get(last_element).unwrap() > *height {
                    right_max_index[last_element] = i - 1;
                    current_stack.pop();
                } else {
                    break;
                }
            }

            current_stack.push(i);
        }

        current_stack = vec![];
        for (i, height) in heights.iter().rev().enumerate() {
            while !current_stack.is_empty() {
                let last_element = *current_stack.last().unwrap();
                if *heights.get(last_element).unwrap() > *height {
                    left_max_index[last_element] = n - i;
                    current_stack.pop();
                } else {
                    break;
                }
            }

            current_stack.push(n - 1 - i);
        }

        let mut max = 0;

        for i in 0..n {
            let area = ((right_max_index[i] - left_max_index[i] + 1) as i32) * heights[i];
            if area > max {
                max = area
            }
        }

        max
    }

    pub fn largest_rectangle_area_2(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut max_area = 0;
        let mut current_stack: Vec<usize> = vec![];
        for i in 0..n + 1 {
            while !current_stack.is_empty()
                && (i == n || heights[*current_stack.last().unwrap()] > heights[i])
            {
                let height = heights[*current_stack.last().unwrap()];
                current_stack.pop();
                let mut width = i as i32;
                if !current_stack.is_empty() {
                    width = (i - 1 - *current_stack.last().unwrap()) as i32
                }
                max_area = max(width * height, max_area);
            }

            current_stack.push(i);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0084::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::largest_rectangle_area_2(vec![2, 1, 5, 6, 2, 3]);
        assert_eq!(ok, 10)
    }

    #[test]
    fn test_2() {
        let ok = Solution::largest_rectangle_area_2(vec![2, 4]);
        assert_eq!(ok, 4)
    }
}
