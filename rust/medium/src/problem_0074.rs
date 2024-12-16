use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left_index = 0;
        let mut right_index = matrix.len() * matrix[0].len() - 1;

        while left_index < right_index {
            let mid = (right_index + left_index) / 2;
            let mid_i = mid / matrix[0].len();
            let mid_j = mid % matrix[0].len();

            match matrix[mid_i][mid_j].cmp(&target) {
                Ordering::Less => {
                    left_index = mid + 1;
                }
                Ordering::Equal => {
                    left_index = mid;
                    break;
                }
                Ordering::Greater => {
                    right_index = mid;
                }
            }
        }
        let i = left_index / matrix[0].len();
        let j = left_index % matrix[0].len();

        matrix[i][j] == target
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0074::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ))
    }

    #[test]
    fn test_2() {
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ))
    }
}
