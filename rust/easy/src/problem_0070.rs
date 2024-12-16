struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ways = vec![0; n as usize + 1];
        ways[0] = 1;
        ways[1] = 1;
        Self::internal_climb_stairs(n as usize, &mut ways);
        ways[n as usize]
    }

    fn internal_climb_stairs(n: usize, ways: &mut Vec<i32>) {
        if n != 0 && ways[n] == 0 {
            if ways[n - 2] == 0 {
                Self::internal_climb_stairs(n - 2, ways)
            }

            if ways[n - 1] == 0 {
                Self::internal_climb_stairs(n - 1, ways)
            }

            ways[n] = ways[n - 1] + ways[n - 2];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0070::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(5), 8)
    }
}
