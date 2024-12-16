use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let n = nums.len();
        nums.sort();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                match sum.cmp(&(0)) {
                    Ordering::Less => {
                        j += 1;
                    }
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;

                        while nums[j] == nums[j - 1] && j < k {
                            j += 1;
                        }
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0015::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        )
    }
}
