struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        let nums1_len = nums1.len();
        if nums2.len() % 2 == 1 {
            for num in nums1 {
                result ^= num;
            }
        }

        if nums1_len % 2 == 1 {
            for num in nums2 {
                result ^= num;
            }
        }

        result
    }
}
