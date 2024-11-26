use std::collections::{HashSet, VecDeque};

struct Solution;

struct Config {
    board: [i32; 6],
}

impl Config {
    pub fn from(initial_config: Vec<Vec<i32>>) -> Self {
        Self {
            board: [
                initial_config[0][0],
                initial_config[0][1],
                initial_config[0][2],
                initial_config[1][0],
                initial_config[1][1],
                initial_config[1][2],
            ],
        }
    }

    pub fn new_configs_from(&self) -> Vec<Self> {
        let mut new_configs = vec![];
        for (index, val) in self.board.iter().enumerate() {
            if *val == 0 {
                if index > 0 && index != 3 {
                    let mut new_board = self.board;
                    new_board[index] = new_board[index - 1];
                    new_board[index - 1] = 0;
                    new_configs.push(Self { board: new_board })
                }
                if index < 5 && index != 2 {
                    let mut new_board = self.board;
                    new_board[index] = new_board[index + 1];
                    new_board[index + 1] = 0;
                    new_configs.push(Self { board: new_board })
                }

                let up_index = index % 3;
                let down_index = up_index + 3;
                let mut new_board = self.board;
                let tmp_up = new_board[up_index];
                new_board[up_index] = new_board[down_index];
                new_board[down_index] = tmp_up;

                new_configs.push(Self { board: new_board })
            }
        }
        new_configs
    }

    pub fn hash(&self) -> i32 {
        let mut result = 0;
        for (index, val) in self.board.iter().enumerate() {
            result += val * 10_i32.pow(index as u32);
        }
        result
    }

    pub fn is_end_config(&self) -> bool {
        self.board[0] == 1
            && self.board[1] == 2
            && self.board[2] == 3
            && self.board[3] == 4
            && self.board[4] == 5
    }
}

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let mut already_seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(Config::from(board));

        let mut level = 0;
        while !queue.is_empty() {
            let queue_elements = queue.len();
            for _ in 0..queue_elements {
                let elem = queue.pop_front().unwrap();
                if elem.is_end_config() {
                    return level;
                }

                if already_seen.insert(elem.hash()) {
                    let new_configs = elem.new_configs_from();
                    for new_config in new_configs {
                        queue.push_back(new_config)
                    }
                }
            }
            level += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0773::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![4, 0, 5]]),
            1
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![1, 2, 3], vec![5, 4, 0]]),
            -1
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![4, 1, 2], vec![5, 0, 3]]),
            5
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::sliding_puzzle(vec![vec![3, 2, 4], vec![1, 5, 0]]),
            14
        )
    }
}
