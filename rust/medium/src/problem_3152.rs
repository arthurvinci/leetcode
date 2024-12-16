use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut deviation = vec![];
        for i in 1..nums.len() {
            if (nums[i - 1] + nums[i]) % 2 == 0 {
                deviation.push(i);
            }
        }
        queries
            .iter()
            .map(|query| {
                if deviation.is_empty() {
                    true
                } else {
                    let mut left_index = 0;
                    let mut right_index = deviation.len();

                    while left_index < right_index {
                        let mid = (left_index + right_index) / 2;
                        match deviation[mid].cmp(&(query[0] as usize)) {
                            Ordering::Less => {
                                left_index = mid + 1;
                            }
                            Ordering::Equal => {
                                left_index = mid + 1;
                                break;
                            }
                            Ordering::Greater => {
                                right_index = mid;
                            }
                        }
                    }

                    left_index == deviation.len() || query[1] < deviation[left_index] as i32
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_3152::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_array_special(vec![2, 2], vec![vec![0, 0]]),
            vec![true]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
            vec![false, true]
        );
    }
}
