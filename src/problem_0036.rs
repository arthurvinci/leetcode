use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut lines = vec![HashSet::<u32>::new(); 9];
        let mut columns = vec![HashSet::<u32>::new(); 9];
        let mut boxes = vec![HashSet::<u32>::new(); 9];

        for (line_index, column_vec) in board.iter().enumerate() {
            for (column_index, cell_value) in column_vec.iter().enumerate() {
                match cell_value.to_digit(10) {
                    None => {}
                    Some(value) => {
                        let box_index = (line_index / 3) * 3 + column_index / 3;
                        if !(lines.get_mut(line_index).unwrap().insert(value)
                            && columns.get_mut(column_index).unwrap().insert(value)
                            && boxes.get_mut(box_index).unwrap().insert(value))
                        {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0036::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert!(ok)
    }

    #[test]
    fn test_2() {
        let ok = Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]);
        assert!(!ok)
    }

    #[test]
    fn test_3() {
        let ok = Solution::is_valid_sudoku(vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ]);
        assert!(!ok)
    }
}
