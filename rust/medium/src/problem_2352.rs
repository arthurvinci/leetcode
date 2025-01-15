use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut hashed_rows = HashMap::new();
        for row in &grid {
            let hash = row.iter().fold(String::new(), |mut acc, num| {
                acc.push(';');
                acc.push_str(&num.to_string());
                acc
            });
            *hashed_rows.entry(hash).or_insert(0) += 1;
        }

        let mut equals = 0;

        for j in 0..grid.len() {
            let mut hash = String::new();
            for i in 0..grid.len() {
                hash.push(';');
                hash.push_str(&grid[i][j].to_string())
            }

            equals += *hashed_rows.entry(hash).or_insert(0);
        }

        equals
    }
}
