struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![0; 2 * nums.len()];

        for (i, val) in nums.iter().enumerate() {
            ret[i] = *val;
            ret[i + nums.len()] = *val;
        }

        ret
    }
}
