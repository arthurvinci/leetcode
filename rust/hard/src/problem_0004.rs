use std::mem::swap;

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let total_elements = nums1.len() + nums2.len();
        let left_partition_amount = ((total_elements + 1) / 2) as i32;

        if nums1.len() > nums2.len() {
            swap(&mut nums1, &mut nums2)
        }

        let mut l_index = 0;
        let mut r_index = nums1.len() as i32;

        while l_index <= r_index {
            let partition_1_index = (l_index + r_index) / 2;
            let partition_2_index = left_partition_amount - partition_1_index;

            let end_part_1_val = if partition_1_index > 0 {
                nums1[partition_1_index as usize - 1]
            } else {
                i32::MIN
            } as f64;
            let next_part_1_val = if partition_1_index < nums1.len() as i32 {
                nums1[partition_1_index as usize]
            } else {
                i32::MAX
            } as f64;
            let end_part_2_val = if partition_2_index > 0 {
                nums2[partition_2_index as usize - 1]
            } else {
                i32::MIN
            } as f64;
            let next_part_2_val = if partition_2_index < nums2.len() as i32 {
                nums2[partition_2_index as usize]
            } else {
                i32::MAX
            } as f64;

            if end_part_1_val <= next_part_2_val && end_part_2_val <= next_part_1_val {
                return if total_elements % 2 == 1 {
                    end_part_1_val.max(end_part_2_val)
                } else {
                    (end_part_1_val.max(end_part_2_val) + next_part_1_val.min(next_part_2_val))
                        / 2.0
                };
            } else if end_part_1_val > next_part_2_val {
                r_index = partition_1_index - 1;
            } else {
                l_index = partition_1_index + 1;
            }
        }

        -1.0
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0004::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4, 6, 8]),
            4.0
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0)
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0)
    }
}
