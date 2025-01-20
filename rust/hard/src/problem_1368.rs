use std::collections::VecDeque;

struct Solution;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut costs = vec![vec![i32::MAX; n]; m];
        let mut deque = VecDeque::new();

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let dir_map = [1, 2, 3, 4];

        costs[0][0] = 0;
        deque.push_back((0, 0, 0));

        while let Some((x, y, cost)) = deque.pop_front() {
            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;

                if new_x < m && new_y < n {
                    let new_cost = cost + if grid[x][y] == dir_map[i] { 0 } else { 1 };
                    if new_cost < costs[new_x][new_y] {
                        costs[new_x][new_y] = new_cost;
                        if grid[x][y] == dir_map[i] {
                            deque.push_front((new_x, new_y, new_cost));
                        } else {
                            deque.push_back((new_x, new_y, new_cost));
                        }
                    }
                }
            }
        }

        costs[m - 1][n - 1]
    }
}
