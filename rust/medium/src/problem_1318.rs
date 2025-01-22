struct Solution;

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let aorb = a | b;
        let mut bit_flips = 0;

        fn i_th_bit(i: usize, x: i32) -> i32 {
            let ith_bit_mask = 1 << i;
            (x & ith_bit_mask) >> i
        }

        for i in 0..32 {
            if i_th_bit(i, aorb) != i_th_bit(i, c) {
                let a_i = i_th_bit(i, a);
                let b_i = i_th_bit(i, b);

                if a_i == 1 && b_i == 1 {
                    bit_flips += 2;
                } else {
                    bit_flips += 1;
                }
            }
        }

        bit_flips
    }
}
