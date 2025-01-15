struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut new_string = String::new();
        let word1_bytes = word1.as_bytes();
        let word2_bytes = word2.as_bytes();

        let min = word2.len().min(word1.len());
        for i in 0..min {
            new_string.push(word1_bytes[i] as char);
            new_string.push(word2_bytes[i] as char);
        }

        for i in min..word1.len() {
            new_string.push(word1_bytes[i] as char)
        }

        for i in min..word2.len() {
            new_string.push(word2_bytes[i] as char);
        }

        new_string
    }
}
