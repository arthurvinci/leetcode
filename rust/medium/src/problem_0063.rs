struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut values = vec![0; n];

        values[n - 1] = 1;

        for i in (0..m).rev() {
            if obstacle_grid[i][n - 1] == 1 {
                values[n - 1] = 0;
            }

            for j in (0..n - 1).rev() {
                if obstacle_grid[i][j] == 0 {
                    values[j] += values[j + 1];
                } else {
                    values[j] = 0;
                }
            }
        }

        values[0]
    }
}
