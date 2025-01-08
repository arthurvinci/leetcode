use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            0
        } else {
            let mut heap: BinaryHeap<(Reverse<i32>, i32, i32)> = BinaryHeap::new();
            let mut visited = HashSet::new();
            visited.insert(0);
            let mut newly_visited = 0;
            let mut min_cost = 0;

            while visited.len() < points.len() {
                for i in 0..points.len() {
                    if i != newly_visited && !visited.contains(&(i as i32)) {
                        let dist = (points[newly_visited][0] - points[i][0]).abs()
                            + (points[newly_visited][1] - points[i][1]).abs();
                        heap.push((Reverse(dist), newly_visited as i32, i as i32));
                    }
                }

                let mut popped = heap.pop().unwrap();

                while !visited.insert(popped.2) {
                    popped = heap.pop().unwrap()
                }

                newly_visited = popped.2 as usize;
                min_cost += popped.0 .0
            }

            min_cost
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_1584::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_cost_connect_points(vec![
                vec![0, 0],
                vec![2, 2],
                vec![3, 10],
                vec![5, 2],
                vec![7, 0]
            ]),
            20
        )
    }
}
