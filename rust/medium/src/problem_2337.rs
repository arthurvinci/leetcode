struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut index_start = 0;
        let mut index_target = 0;
        let start: Vec<char> = start.chars().collect();
        let target: Vec<char> = target.chars().collect();

        loop {
            while index_start < start.len() && start[index_start] == '_' {
                index_start += 1;
            }

            while index_target < target.len() && target[index_target] == '_' {
                index_target += 1;
            }

            if index_start < start.len() && index_target < target.len() {
                if start[index_start] != target[index_target] {
                    return false;
                } else {
                    if start[index_start] == 'L' && index_start < index_target {
                        return false;
                    }

                    if start[index_start] == 'R' && index_start > index_target {
                        return false;
                    }
                }

                index_target += 1;
                index_start += 1;
            } else {
                break;
            }
        }

        while index_start < start.len() {
            if start[index_start] != '_' {
                return false;
            } else {
                index_start += 1;
            }
        }

        while index_target < target.len() {
            if target[index_target] != '_' {
                return false;
            } else {
                index_target += 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_2337::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::can_change(
            "_L__R__R_".to_string(),
            "L______RR".to_string()
        ));
    }
}
