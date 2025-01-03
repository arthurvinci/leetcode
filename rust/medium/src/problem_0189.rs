struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;

        if k != 0 && n > 1 {
            nums.reverse();
            Self::local_reverse(nums, 0, k - 1);
            Self::local_reverse(nums, k, n - 1);
        }
    }

    fn local_reverse(nums: &mut [i32], mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}
