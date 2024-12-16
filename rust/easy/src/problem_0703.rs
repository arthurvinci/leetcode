use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut new = BinaryHeap::from(
            nums.iter()
                .map(|x| Reverse(*x))
                .collect::<Vec<Reverse<i32>>>(),
        );
        while new.len() > k as usize {
            new.pop();
        }
        Self {
            heap: new,
            k: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
