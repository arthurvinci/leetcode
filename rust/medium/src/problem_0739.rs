struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];
        let mut stack = vec![];
        for (index, temperature) in temperatures.into_iter().enumerate() {
            loop {
                if let Some((i, temp)) = stack.pop() {
                    if temperature > temp {
                        ans[i] = (index - i) as i32;
                    } else {
                        stack.push((i, temp));
                        break;
                    }
                } else {
                    break;
                }
            }
            stack.push((index, temperature))
        }
        ans
    }
}
