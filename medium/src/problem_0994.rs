use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut fresh_oranges = 0;
        let mut rotten_oranges: VecDeque<(usize, usize)> = VecDeque::new();
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    fresh_oranges += 1;
                } else if grid[i][j] == 2 {
                    rotten_oranges.push_back((i, j));
                    fresh_oranges += 1;
                }
            }
        }

        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut time = -1;
        while !rotten_oranges.is_empty() {
            let amount = rotten_oranges.len();
            for _ in 0..amount {
                let (i, j) = rotten_oranges.pop_front().unwrap();
                fresh_oranges -= 1;

                for (offset_i, offset_j) in offsets {
                    let new_i = i as i32 + offset_i;
                    let new_j = j as i32 + offset_j;

                    if new_i >= 0 && new_i < m as i32 && new_j >= 0 && new_j < n as i32 {
                        let i_u = new_i as usize;
                        let j_u = new_j as usize;

                        if grid[i_u][j_u] == 1 {
                            rotten_oranges.push_back((i_u, j_u));
                            grid[i_u][j_u] = 2;
                        }
                    }
                }
            }
            time += 1;
        }

        if fresh_oranges > 0 {
            -1
        } else {
            time
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0994::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        )
    }
}
