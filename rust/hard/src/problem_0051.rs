use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
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

        working_positions
            .iter()
            .map(|x| Self::boardify(x, n))
            .collect()
    }

    fn boardify(positions: &Vec<usize>, n: usize) -> Vec<String> {
        let mut ret = vec![];

        for position in positions {
            let mut str = String::new();
            for _ in 0..*position {
                str.push('.');
            }
            str.push('Q');
            for _ in *position + 1..n {
                str.push('.');
            }
            ret.push(str);
        }

        ret
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

#[cfg(test)]
mod test {
    use crate::problem_0051::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::solve_n_queens(1).len(), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::solve_n_queens(2).len(), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::solve_n_queens(3).len(), 0);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::solve_n_queens(4).len(), 2);
    }
}
