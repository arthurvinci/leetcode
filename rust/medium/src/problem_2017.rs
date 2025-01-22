struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut up_prefix = vec![0i64; n];
        let mut down_prefix = vec![0i64; n];
        up_prefix[0] = grid[0][0] as i64;
        down_prefix[0] = grid[1][0] as i64;

        for i in 1..n {
            up_prefix[i] = up_prefix[i - 1] + grid[0][i] as i64;
            down_prefix[i] = down_prefix[i - 1] + grid[1][i] as i64;
        }

        let mut result = i64::MAX;
        for i in 0..n {
            let max_collected_up = up_prefix[n - 1] - up_prefix[i];
            let max_collected_down = down_prefix[i] - grid[1][i] as i64;
            result = result.min(max_collected_down.max(max_collected_up));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2017::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]);
        assert_eq!(ok, 4)
    }
}
