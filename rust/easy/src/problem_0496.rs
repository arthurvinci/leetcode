use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut num_to_index_1 = HashMap::new();
        for (index, num) in nums1.iter().enumerate() {
            num_to_index_1.insert(*num, index);
        }

        let mut ans = vec![-1; nums1.len()];

        let mut stack = vec![];
        for num in nums2.into_iter().rev() {
            loop {
                let top = stack.pop();
                if let Some(top) = top {
                    if num < top {
                        stack.push(top);
                        break;
                    }
                } else {
                    break;
                }
            }

            if !stack.is_empty() {
                if let Some(index) = num_to_index_1.get(&num) {
                    let biggest = *stack.last().unwrap();
                    ans[*index] = biggest;
                }
            }

            stack.push(num)
        }

        ans
    }
}
