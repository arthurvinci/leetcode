use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            1
        } else {
            let mut left_index = 0;
            let mut max_length = 1;
            let mut next_bigger = None;

            for right_index in 1..arr.len() {
                match arr[right_index].cmp(&arr[right_index - 1]) {
                    Ordering::Less => {
                        let bigger = next_bigger.unwrap_or(false);
                        if bigger {
                            left_index = right_index - 1;
                        } else {
                            max_length = max_length.max(right_index - left_index + 1);
                        }
                        next_bigger = Some(true);
                    }
                    Ordering::Equal => {
                        left_index = right_index;
                        next_bigger = None;
                    }
                    Ordering::Greater => {
                        let bigger = next_bigger.unwrap_or(true);
                        if bigger {
                            max_length = max_length.max(right_index - left_index + 1);
                        } else {
                            left_index = right_index - 1;
                        }
                        next_bigger = Some(false);
                    }
                }
            }

            max_length as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0978::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_turbulence_size(vec![0, 1, 1, 0, 1, 0, 1, 1, 0, 0]),
            5
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
    }
}
