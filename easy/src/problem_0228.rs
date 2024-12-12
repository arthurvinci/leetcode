struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ranges = vec![];

        if !nums.is_empty() {
            let mut left_index = 0;
            let mut right_index = 0;

            while right_index < nums.len() - 1 {
                if nums[right_index + 1] != nums[right_index] + 1 {
                    ranges.push(Self::output_string(&nums, left_index, right_index));
                    right_index += 1;
                    left_index = right_index;
                } else {
                    right_index += 1;
                }
            }

            ranges.push(Self::output_string(&nums, left_index, right_index))
        }

        ranges
    }

    fn output_string(nums: &[i32], left_index: usize, right_index: usize) -> String {
        if left_index == right_index {
            format!("{}", nums[left_index])
        } else {
            format!("{}->{}", nums[left_index], nums[right_index])
        }
    }
}
