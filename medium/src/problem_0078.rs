struct Solution;

impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            vec![vec![]]
        } else {
            let new_val = nums.pop().unwrap();
            let mut ret = Self::subsets(nums);
            let mut added_values = vec![];
            for subset in &ret {
                let mut copy = subset.clone();
                copy.push(new_val);
                added_values.push(copy);
            }
            ret.append(&mut added_values);

            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0078::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
