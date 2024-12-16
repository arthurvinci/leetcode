struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut zipped: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        zipped.sort();
        let mut fleets = 0;

        let mut current_eta = 0f64;

        while let Some((position, speed)) = zipped.pop() {
            let this_eta = ((target - position) as f64) / (speed as f64);
            if current_eta < this_eta {
                fleets += 1;
                current_eta = this_eta;
            }
        }

        fleets
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0853::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        )
    }
}
