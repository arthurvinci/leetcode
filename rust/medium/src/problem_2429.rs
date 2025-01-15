struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut bits = num2.count_ones();
        let mut x = 0;
        for i in (0..32).rev() {
            if (num1 >> i) & 1 == 1 && bits > 0 {
                x += 1 << i;
                bits -= 1;
            }
        }
        let mut i = 0;
        while bits > 0 {
            if (num1 >> i) & 1 == 0 {
                x += 1 << i;
                bits -= 1;
            }
            i += 1;
        }

        x
    }
}
