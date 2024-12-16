use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;
impl Solution {
    pub fn minimum_obstacles(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut q = BinaryHeap::from([Reverse((grid[0][0], 0, 0, 0))]);
        while let Some(Reverse((k, d, i, j))) = q.pop() {
            if (i, j) == (n - 1, m - 1) {
                return k;
            }
            for &(ii, jj) in &[
                (i, j.wrapping_sub(1)),
                (i, j + 1),
                (i.wrapping_sub(1), j),
                (i + 1, j),
            ] {
                if ii < n && jj < m && grid[ii][jj] != 2 {
                    q.push(Reverse((k + grid[ii][jj], d + 1, ii, jj)));
                    grid[ii][jj] = 2;
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2290::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::minimum_obstacles(vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            2
        );
    }
}
