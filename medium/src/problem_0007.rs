struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x.abs();
        let mut d = [0_u8; 10];
        let mut i = 0;
        loop {
            d[i] = (n % 10) as u8;
            n = n / 10;
            i += 1;
            if n == 0 {
                break;
            }
        }
        for j in 0..i {
            if let Some(k) = n.checked_mul(10) {
                if let Some(l) = k.checked_add(d[j] as i32) {
                    n = l;
                    continue;
                }
            };
            return 0;
        }
        n * if x < 0 { -1 } else { 1 }
    }
}
