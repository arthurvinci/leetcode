use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::from_iter(
            nums.into_iter()
                .enumerate()
                .map(|(index, val)| Reverse((val, index))),
        );

        for _ in 0..k {
            let (val, index) = heap.pop().unwrap().0;
            heap.push(Reverse((val * multiplier, index)));
        }

        let mut final_result = vec![0; heap.len()];

        for Reverse((val, index)) in heap.into_iter() {
            final_result[index] = val;
        }

        final_result
    }
}
