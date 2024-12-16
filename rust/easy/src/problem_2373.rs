use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut max_local: Vec<Vec<i32>> = vec![vec![0; grid.len() - 2]; grid.len() - 2];
        for (i, vec_i) in grid.iter().enumerate() {
            let i = i as i32;
            for (j, value) in vec_i.iter().enumerate() {
                let j = j as i32;

                let min_index_i = max(i - 2, 0);
                let max_index_i = min(i + 1, grid.len() as i32 - 2);
                let min_index_j = max(j - 2, 0);
                let max_index_j = min(j + 1, grid.len() as i32 - 2);

                for index_i in min_index_i..max_index_i {
                    for index_j in min_index_j..max_index_j {
                        let i = index_i as usize;
                        let j = index_j as usize;

                        max_local[i][j] = max(max_local[i][j], *value)
                    }
                }
            }
        }

        max_local
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2373::Solution;

    #[test]
    fn test_1() {
        let input = vec![
            vec![9, 9, 8, 1],
            vec![5, 6, 2, 6],
            vec![8, 2, 6, 4],
            vec![6, 2, 2, 2],
        ];
        let expected_output = vec![vec![9, 9], vec![8, 6]];

        assert_eq!(Solution::largest_local(input), expected_output)
    }

    #[test]
    fn test_2() {
        let input = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 2, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected_output = vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]];

        assert_eq!(Solution::largest_local(input), expected_output);
    }
}
