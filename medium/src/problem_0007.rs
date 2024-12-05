struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x.abs();
        let mut d = [0_u8; 10];
        let mut i = 0;
        loop {
            d[i] = (n % 10) as u8;
            n /= 10;
            i += 1;
            if n == 0 {
                break;
            }
        }
        for digit in d.iter().take(i) {
            if let Some(k) = n.checked_mul(10) {
                if let Some(l) = k.checked_add(*digit as i32) {
                    n = l;
                    continue;
                }
            };
            return 0;
        }
        n * if x < 0 { -1 } else { 1 }
    }
}
