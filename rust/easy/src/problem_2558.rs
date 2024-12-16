use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(gifts);
        for _ in 0..k {
            let max_gift = heap.pop().unwrap();
            let root = (max_gift as f64).sqrt().floor() as i32;
            heap.push(root)
        }

        let sum: i32 = heap.into_iter().sum();
        sum as i64
    }
}
