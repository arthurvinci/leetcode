struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut nb_zero = 0;
        let mut product = 1;
        for num in &nums {
            if *num == 0 {
                nb_zero += 1;
            } else {
                product *= *num;
            }
        }

        if nb_zero > 0 {
            let mut ret = vec![0; nums.len()];
            if nb_zero == 1 {
                for (i, num) in nums.iter().enumerate() {
                    if *num == 0 {
                        ret[i] = product;
                        break;
                    }
                }
            }

            ret
        } else {
            nums.iter().map(|x| product / *x).collect()
        }
    }
}
