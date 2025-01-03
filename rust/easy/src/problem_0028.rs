struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_vec: Vec<char> = needle.chars().collect();
        let haystack_vec: Vec<char> = haystack.chars().collect();
        for i in 0..haystack.len() {
            for j in 0..needle.len() {
                if !(i + j < haystack.len() && needle_vec[j] == haystack_vec[i + j]) {
                    break;
                }

                if j == needle.len() - 1 {
                    return i as i32;
                }
            }
        }
        -1
    }
}
