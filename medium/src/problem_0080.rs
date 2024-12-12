struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;

        let mut same_int = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[i - k - 1] {
                same_int += 1;
            } else {
                same_int = 1;
            }
            nums.swap(i, i - k);

            if same_int > 2 {
                k += 1;
            }
        }

        (nums.len() - k) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0080::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 2, 3],),
            5
        )
    }
}
