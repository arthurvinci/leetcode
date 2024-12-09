use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut elements = HashSet::new();

        for num in nums {
            elements.insert(num);
        }

        let mut longest_subsequence = 0;
        for num in &elements {
            let mut subsequence = 1;
            let mut nb = *num;

            if !elements.contains(&(nb - 1)) {
                while elements.contains(&(nb + 1)) {
                    nb += 1;
                    subsequence += 1;
                }
            }
            longest_subsequence = longest_subsequence.max(subsequence);
        }

        longest_subsequence
    }
}
