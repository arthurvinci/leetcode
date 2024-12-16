struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut values = vec![0; n as usize];
        values[n as usize - 1] = 1;

        for _ in (0..m).rev() {
            for i in (0..(n as usize - 1)).rev() {
                values[i] += values[i + 1]
            }
        }

        values[0]
    }
}
