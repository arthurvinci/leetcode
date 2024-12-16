struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();
        let mut board = board;
        let word: Vec<char> = word.chars().collect();
        for i in 0..m {
            for j in 0..n {
                if Solution::dfs(&mut board, &word, i, j) {
                    return true;
                }
            }
        }
        false
    }
    fn dfs(board: &mut Vec<Vec<char>>, word: &[char], x: usize, y: usize) -> bool {
        if word.len() == 1 {
            return board[x][y] == word[0];
        }
        if board[x][y] != word[0] {
            return false;
        }
        let dirs = [[-1, 0], [0, 1], [1, 0], [0, -1]];
        board[x][y] = '*';
        for dir in dirs {
            let x = x as i32 + dir[0];
            let y = y as i32 + dir[1];
            if x < 0 || y < 0 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if x >= board.len() || y >= board[0].len() {
                continue;
            }
            if Solution::dfs(board, &word[1..], x, y) {
                board[x][y] = word[0];
                return true;
            }
        }
        board[x][y] = word[0];
        false
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0079::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ))
    }

    #[test]
    fn test_2() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'E', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCEFSADEESE".to_string()
        ))
    }
}
