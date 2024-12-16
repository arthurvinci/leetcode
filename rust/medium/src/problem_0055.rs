struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut cache = vec![None; nums.len()];
        cache[nums.len() - 1] = Some(true);
        Self::try_jump(&nums, 0, &mut cache);
        cache[0].unwrap()
    }

    fn try_jump(nums: &[i32], position: usize, cache: &mut [Option<bool>]) {
        let jump_size = nums[position] as usize;
        let mut res = false;
        for i in position + 1..nums.len().min(position + jump_size + 1) {
            if cache[i].is_none() {
                Self::try_jump(nums, i, cache);
            }
            if cache[i].unwrap() {
                res = true;
                break;
            }
        }
        cache[position] = Some(cache[position].unwrap_or(false) || res);
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0055::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4],))
    }
}
