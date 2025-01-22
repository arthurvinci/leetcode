struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut min_cost = vec![i32::MAX; cost.len()];
        min_cost[cost.len() - 1] = cost[cost.len() - 1];
        min_cost[cost.len() - 2] = cost[cost.len() - 2];

        Self::compute_min_cost(0, &mut min_cost, &cost);

        min_cost[0].min(min_cost[1])
    }

    fn compute_min_cost(index: usize, min_cost: &mut [i32], cost: &[i32]) {
        if min_cost[index] == i32::MAX {
            let mut min = i32::MAX;
            if index + 1 < min_cost.len() {
                if min_cost[index + 1] == i32::MAX {
                    Self::compute_min_cost(index + 1, min_cost, cost);
                }
                min = min.min(min_cost[index + 1]);
            }

            if index + 2 < min_cost.len() {
                if min_cost[index + 2] == i32::MAX {
                    Self::compute_min_cost(index + 2, min_cost, cost);
                }
                min = min.min(min_cost[index + 2]);
            }

            min_cost[index] = min + cost[index];
        }
    }
}
