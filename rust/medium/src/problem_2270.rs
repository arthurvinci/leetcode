struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let total_sum: i64 = nums.iter().map(|&x| x as i64).sum::<i64>();

        let mut nb_ways = 0;
        let mut current_sum: i64 = 0;
        for i in 0..(nums.len() - 1) {
            current_sum += nums[i] as i64;
            if current_sum >= total_sum - current_sum {
                nb_ways += 1;
            }
        }

        nb_ways
    }
}

#[cfg(test)]
mod test {
    use crate::problem_2270::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::ways_to_split_array(vec![10, 4, -8, 7]), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::ways_to_split_array(vec![2, 3, 1, 0]), 2)
    }
}
