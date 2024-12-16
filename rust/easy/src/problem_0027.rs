struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let mut i = 0;
        let mut last_element = nums.len() as i32 - 1;

        while i as i32 <= last_element {
            if nums[i] == val {
                nums.swap(i, last_element as usize);
                last_element -= 1;
            } else {
                i += 1;
            }
        }

        i as i32
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0027::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::remove_element(&mut vec![3, 2, 2, 3], 3);
        assert_eq!(ok, 2)
    }
}
