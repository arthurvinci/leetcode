struct Solution;
impl Solution {
    pub fn can_complete_circuit(mut gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        gas.append(&mut gas.clone());

        let mut left_index = 0;
        let mut stock = gas[0] - cost[0];

        for right_index in 1..gas.len() {
            let true_index = right_index % cost.len();

            if stock < 0 {
                stock = gas[true_index] - cost[true_index];
                left_index = true_index;
            } else {
                stock += gas[true_index] - cost[true_index];
                if true_index == left_index {
                    return true_index as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0134::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        )
    }
}
