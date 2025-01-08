use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    small_values: BinaryHeap<i32>,
    big_values: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            small_values: BinaryHeap::new(),
            big_values: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.small_values.is_empty() {
            self.small_values.push(num);
        } else {
            if num <= *self.small_values.peek().unwrap() {
                self.small_values.push(num)
            } else {
                self.big_values.push(Reverse(num))
            }

            if self.small_values.len() == self.big_values.len() + 2 {
                let top = self.small_values.pop().unwrap();
                self.big_values.push(Reverse(top))
            } else if self.big_values.len() > self.small_values.len() {
                let top = self.big_values.pop().unwrap();
                self.small_values.push(top.0);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small_values.len() > self.big_values.len() {
            *self.small_values.peek().unwrap() as f64
        } else {
            (*self.small_values.peek().unwrap() as f64 + self.big_values.peek().unwrap().0 as f64)
                / 2.0
        }
    }
}
