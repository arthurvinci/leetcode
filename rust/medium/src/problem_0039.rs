struct Solution;

impl Solution {
    fn find_combinations(
        index: usize,
        target: i32,
        candidates: &Vec<i32>,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current.clone());
            return;
        }

        for i in index..candidates.len() {
            if candidates[i] <= target {
                current.push(candidates[i]);

                Self::find_combinations(i, target - candidates[i], candidates, current, result);

                current.pop();
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::find_combinations(0, target, &candidates, &mut current, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0039::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn test_2() {
        let mut sol = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        sol.sort();
        assert_eq!(sol, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_3() {
        let mut sol = Solution::combination_sum(vec![2, 3], 6);
        sol.sort();
        assert_eq!(sol, vec![vec![2, 2, 2], vec![3, 3]]);
    }
}
