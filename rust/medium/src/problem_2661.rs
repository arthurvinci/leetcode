use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut num_to_index = HashMap::new();

        for (i, line) in mat.iter().enumerate() {
            for (j, num) in line.iter().enumerate() {
                num_to_index.insert(*num, (i, j));
            }
        }

        let m = mat.len();
        let n = mat[0].len();

        let mut line_count = vec![0; m];
        let mut column_count = vec![0; n];

        for (index, num) in arr.iter().enumerate() {
            let (i, j) = num_to_index.get(&num).unwrap();
            line_count[*i] += 1;
            if line_count[*i] == n {
                return index as i32;
            }

            column_count[*j] += 1;
            if column_count[*j] == m {
                return index as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2661::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]);
        assert_eq!(ok, 2)
    }
}
