struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut current_altitude = 0;
        let mut max_altitude = 0;

        for num in gain {
            current_altitude += num;
            max_altitude = max_altitude.max(current_altitude);
        }

        max_altitude
    }
}
