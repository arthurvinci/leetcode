struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut prefix_common_array: Vec<i32> = vec![];
        let mut a_bit_mask: u64 = 0;
        let mut b_bit_mask: u64 = 0;
        for (a_num, b_num) in a.into_iter().zip(b.into_iter()) {
            let mut previous = *prefix_common_array.last().unwrap_or(&0) as u64;

            // 0..010..0
            //     ^ a_num-th bit
            let a_num_mask = 1 << a_num;
            // Store the result in the bit mask
            a_bit_mask |= a_num_mask;

            // Adds one to previous if both a_num-th bit is equal to 1
            previous += ((a_bit_mask & a_num_mask) & (b_bit_mask & a_num_mask)) >> a_num;

            let b_num_mask = 1 << b_num;
            b_bit_mask |= b_num_mask;

            previous += ((a_bit_mask & b_num_mask) & (b_bit_mask & b_num_mask)) >> b_num;

            prefix_common_array.push(previous as i32);
        }

        prefix_common_array
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2657::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
            vec![0, 2, 3, 4]
        )
    }
}
