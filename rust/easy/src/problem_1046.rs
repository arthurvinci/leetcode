use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        while heap.len() > 1 {
            let mut stone_1 = heap.pop().unwrap();
            let stone_2 = heap.pop().unwrap();
            stone_1 -= stone_2;
            if stone_1 > 0 {
                heap.push(stone_1)
            }
        }

        *heap.peek().unwrap()
    }
}
