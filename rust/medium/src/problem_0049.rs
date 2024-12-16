struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        let n = citations.len();
        citations.sort_unstable();
        for (i, &citation) in citations.iter().enumerate() {
            if citation as usize >= n - i {
                return (n - i) as i32;
            }
        }
        0
    }
}
