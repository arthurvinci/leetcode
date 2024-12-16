struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // First transpose
        for i in 0..n {
            for j in i..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        // Then swap columns
        for line in matrix.iter_mut() {
            for j in 0..(n / 2) {
                line.swap(j, n - 1 - j);
            }
        }
    }
}
