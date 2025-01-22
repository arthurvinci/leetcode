struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut kept_intervals: Vec<&Vec<i32>> = vec![];
        intervals.sort_by(|i1, i2| (i1[0], i1[1]).cmp(&(i2[0], i2[1])));

        for interval in &intervals {
            if let Some(top_interval) = kept_intervals.pop() {
                if top_interval[1] > interval[0] {
                    if top_interval[1] > interval[1] {
                        kept_intervals.push(interval);
                    } else {
                        kept_intervals.push(top_interval);
                    }
                } else {
                    kept_intervals.push(top_interval);
                    kept_intervals.push(interval);
                }
            } else {
                kept_intervals.push(interval)
            }
        }

        (intervals.len() - kept_intervals.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0435::Solution;

    #[test]
    fn test_1() {
        let ok =
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]);
        assert_eq!(ok, 1)
    }
}
