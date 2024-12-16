use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut working_positions = vec![];

        for i in 0..n {
            let mut positions = vec![vec![i]];

            let mut columns_used = 1;
            while columns_used < n {
                let mut new_positions = vec![];
                for position in positions.clone().into_iter() {
                    let available_lines = Self::available_lines(&position, n);
                    for available_line in available_lines {
                        let mut new_position = position.clone();
                        new_position.push(available_line);
                        new_positions.push(new_position);
                    }
                }
                if new_positions.is_empty() {
                    break;
                } else {
                    positions = new_positions;
                }

                columns_used += 1;
            }

            if columns_used == n {
                working_positions.append(&mut positions);
            }
        }
        working_positions.len() as i32
    }

    fn available_lines(used_lines: &[usize], n: usize) -> Vec<usize> {
        let mut available_lines: HashSet<usize> = HashSet::from_iter(0..n);
        let cur_column = used_lines.len();
        for (colum, used_line) in used_lines.iter().enumerate() {
            available_lines.remove(used_line);
            let dist = cur_column - colum;

            if *used_line >= dist {
                available_lines.remove(&(used_line - dist));
            }

            if used_line + dist < n {
                available_lines.remove(&(used_line + dist));
            }
        }

        available_lines.into_iter().collect()
    }
}
