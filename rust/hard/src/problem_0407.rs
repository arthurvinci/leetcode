use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut to_visit = BinaryHeap::new();

        for i in 0..m {
            to_visit.push((Reverse(height_map[i][0]), i, 0));

            visited[i][0] = true;

            to_visit.push((Reverse(height_map[i][n - 1]), i, 0));

            visited[i][n - 1] = true;
        }

        for j in 1..n - 1 {
            to_visit.push((Reverse(height_map[0][j]), 0, j));
            visited[0][j] = true;
            to_visit.push((Reverse(height_map[m - 1][j]), m - 1, j));
            visited[m - 1][j] = true;
        }

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut trapped = 0;

        while let Some((Reverse(height), i, j)) = to_visit.pop() {
            for (dx, dy) in directions {
                let new_i = (i as i32) + dx;
                let new_j = (j as i32) + dy;

                if new_i >= 0 && new_i < m as i32 && new_j >= 0 && new_j < n as i32 {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;

                    if !visited[new_i][new_j] {
                        trapped += (height - height_map[new_i][new_j]).max(0);
                        to_visit.push((
                            Reverse(height.max(height_map[new_i][new_j])),
                            new_i,
                            new_j,
                        ));
                        visited[new_i][new_j] = true;
                    }
                }
            }
        }

        trapped
    }
}
