struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let n = events.len();

        // Step 1: Sort the events by their start time
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));

        // Step 2: Create the suffix array for the maximum event value from each event onward
        let mut suffix_max = vec![0; n];
        suffix_max[n - 1] = events[n - 1][2]; // Initialize the last event's value

        // Populate the suffix_max array
        for i in (0..n - 1).rev() {
            suffix_max[i] = suffix_max[i + 1].max(events[i][2]);
        }

        // Step 3: For each event, find the next event that starts after it ends
        let mut max_sum = 0;

        for i in 0..n {
            let mut left = i + 1;
            let mut right = n - 1;
            let mut next_event_index = -1;

            // Perform binary search to find the next non-overlapping event
            while left <= right {
                let mid = left + (right - left) / 2;
                if events[mid][0] > events[i][1] {
                    next_event_index = mid as i32;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }

            // If a valid next event is found, update max_sum
            if next_event_index != -1 {
                let next_event_index = next_event_index as usize;
                max_sum = max_sum.max(events[i][2] + suffix_max[next_event_index]);
            }

            // Also consider the case where we take only the current event
            max_sum = max_sum.max(events[i][2]);
        }

        max_sum
    }
}

#[cfg(test)]
mod test {
    use crate::problem_2054::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]),
            5
        )
    }
}
