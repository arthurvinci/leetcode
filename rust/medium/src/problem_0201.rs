struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let xor = left ^ right;
        let first_different_index = 32 - xor.leading_zeros();
        (left >> first_different_index) << first_different_index
    }
}
