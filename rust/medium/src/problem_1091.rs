use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let n = grid.len();

        if grid[0][0] == 0 {
            heap.push(Reverse((0, 0i32, 0i32)));
        }

        while !heap.is_empty() {
            let (dist, i, j) = heap.pop().unwrap().0;
            if i as usize == n - 1 && j as usize == n - 1 {
                return dist + 1;
            } else {
                grid[i as usize][j as usize] = -1;
                for i_offset in -1..2 {
                    let new_i = i + i_offset;
                    if new_i >= 0 && new_i < n as i32 {
                        for j_offset in -1..2 {
                            let new_j = j + j_offset;
                            if new_j >= 0
                                && new_j < n as i32
                                && grid[new_i as usize][new_j as usize] == 0
                            {
                                grid[new_i as usize][new_j as usize] = -1;
                                heap.push(Reverse((dist + 1, new_i, new_j)));
                            }
                        }
                    }
                }
            }
        }
        -1
    }
}
