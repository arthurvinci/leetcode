struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sums = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            sums[i][0] = matrix[i][0];
            if i > 0 {
                sums[i][0] += sums[i - 1][0];
            }
        }

        for j in 0..matrix[0].len() {
            sums[0][j] = matrix[0][j];
            if j > 0 {
                sums[0][j] += sums[0][j - 1];
            }
        }

        for i in 1..matrix.len() {
            for j in 1..matrix[i].len() {
                sums[i][j] = matrix[i][j] + sums[i - 1][j] + sums[i][j - 1] - sums[i - 1][j - 1];
            }
        }

        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        let mut raw_sum = self.sums[row2][col2];

        if row1 > 0 {
            raw_sum -= self.sums[row1 - 1][col2];
        }

        if col1 > 0 {
            raw_sum -= self.sums[row2][col1 - 1];
        }

        if row1 > 0 && col1 > 0 {
            raw_sum += self.sums[row1 - 1][col1 - 1];
        }

        raw_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0304::NumMatrix;

    #[test]
    fn test_1() {
        let num_array = NumMatrix::new(vec![vec![-4, -5]]);
        assert_eq!(num_array.sum_region(0, 0, 0, 0), -4)
    }

    #[test]
    fn test_2() {
        let num_array = NumMatrix::new(vec![
            vec![8, -4, 5],
            vec![-1, 4, 4],
            vec![-2, 3, 1],
            vec![-4, 4, 3],
        ]);
        assert_eq!(num_array.sum_region(0, 1, 0, 2), 1);
        assert_eq!(num_array.sum_region(1, 1, 1, 2), 8);
        assert_eq!(num_array.sum_region(0, 1, 0, 2), 1);
    }
}
