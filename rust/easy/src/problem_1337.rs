struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else if n == 2 {
            1
        } else {
            let mut i = 2;
            let mut t_0 = 0;
            let mut t_1 = 1;
            let mut t_2 = 1;

            while i < n {
                let new_res = t_0 + t_1 + t_2;
                t_0 = t_1;
                t_1 = t_2;
                t_2 = new_res;
                i += 1;
            }

            t_2
        }
    }
}
