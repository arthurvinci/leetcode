struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 0;
        let mut last_seen = -1004;
        let mut swaps = vec![];
        for (i, val) in nums.iter().enumerate() {
            if *val == last_seen {
                k += 1
            } else {
                last_seen = *val;
                swaps.push(vec![i, i - k]);
            }
        }

        for swap in swaps {
            nums.swap(swap[0], swap[1])
        }

        (nums.len() - k) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0026::Solution;

    #[test]
    fn test_1() {
        let ok = Solution::remove_duplicates(&mut vec![1, 1, 2]);
        assert_eq!(ok, 2)
    }
}
