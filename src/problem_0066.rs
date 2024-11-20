struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut index = digits.len() as i64 - 1;
        loop {
            if index == -1 {
                digits.insert(0, 1);
                break;
            } else {
                let ret = (digits[index as usize] + 1) / 10;
                digits[index as usize] = (digits[index as usize] + 1) % 10;

                if ret == 1 {
                    index -= 1;
                } else {
                    break;
                }
            }
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0066::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0])
    }
}
