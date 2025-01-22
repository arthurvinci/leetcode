use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut queue = VecDeque::new();
        let neighbours = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    queue.push_back((i, j));
                    is_water[i][j] = 0;
                } else {
                    is_water[i][j] = -1;
                }
            }
        }

        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();

            for neighbour in neighbours {
                let new_i = i as i32 + neighbour.0;
                let new_j = j as i32 + neighbour.1;

                if new_i >= 0 && new_j >= 0 {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;

                    if new_i < m && new_j < n && is_water[new_i][new_j] == -1 {
                        is_water[new_i][new_j] = is_water[i][j] + 1;
                        queue.push_back((new_i, new_j))
                    }
                }
            }
        }

        is_water
    }
}
