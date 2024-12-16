use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequence = HashMap::new();
        for num in nums {
            let freq = frequence.entry(num).or_insert(0);
            *freq += 1;
        }

        let mut heap = BinaryHeap::from_iter(
            frequence
                .into_iter()
                .map(|key_value| (key_value.1, key_value.0))
                .into_iter(),
        );
        let mut ret = vec![];

        for _ in 0..k {
            ret.push(heap.pop().unwrap().1)
        }

        ret
    }
}
