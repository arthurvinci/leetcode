struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_index = 0;
        let mut right_index = height.len() - 1;
        let mut contained_water = 0;
        while left_index < right_index {
            let current_height = height[left_index].min(height[right_index]);
            let base = right_index - left_index;
            contained_water = contained_water.max(base as i32 * current_height);

            if height[left_index] <= height[right_index] {
                left_index += 1;
            } else {
                right_index -= 1;
            }
        }

        contained_water
    }
}
