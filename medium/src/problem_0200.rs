struct Solution {}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut counter = 0;

        let rows = grid.len();
        let colums = grid[0].len();

        for i in 0..rows {
            for j in 0..colums {
                if grid[i][j] == '1' {
                    counter += 1;
                    Self::find_all_neighbouring_land(&mut grid, i, j, rows, colums)
                }
            }
        }

        counter
    }

    fn find_all_neighbouring_land(
        grid: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        rows: usize,
        columns: usize,
    ) {
        if grid[i][j] == '1' {
            grid[i][j] = '2';

            if i > 0 {
                Self::find_all_neighbouring_land(grid, i - 1, j, rows, columns);
            }
            if j > 0 {
                Self::find_all_neighbouring_land(grid, i, j - 1, rows, columns);
            }
            if i + 1 < rows {
                Self::find_all_neighbouring_land(grid, i + 1, j, rows, columns);
            }
            if j + 1 < columns {
                Self::find_all_neighbouring_land(grid, i, j + 1, rows, columns);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0200::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ]);
        assert_eq!(ok, 1)
    }

    #[test]
    fn test_2() {
        let ok = Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ]);
        assert_eq!(ok, 3)
    }
}
