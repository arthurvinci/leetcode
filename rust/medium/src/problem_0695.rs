struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;

        for i in 0..=m {
            for j in 0..=n {
                if grid[i][j] == 1 {
                    max = Self::dfs(&mut grid, i as i32, j as i32, m, n).max(max);
                }
            }
        }

        max
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, m: usize, n: usize) -> i32 {
        if i < 0 || i as usize > m {
            return 0;
        }

        if j < 0 || j as usize > n {
            return 0;
        }

        if grid[i as usize][j as usize] == 1 {
            grid[i as usize][j as usize] = -1;
            let mut count = 1;

            count += Self::dfs(grid, i + 1, j, m, n);
            count += Self::dfs(grid, i - 1, j, m, n);
            count += Self::dfs(grid, i, j + 1, m, n);
            count += Self::dfs(grid, i, j - 1, m, n);

            count
        } else {
            0
        }
    }
}
