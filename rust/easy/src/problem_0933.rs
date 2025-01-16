use std::collections::VecDeque;

struct RecentCounter {
    internal_queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self {
            internal_queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        let mut front = self.internal_queue.pop_front();
        loop {
            if let Some(time) = front {
                if t - time <= 3000 {
                    break;
                }
            } else {
                break;
            }
            front = self.internal_queue.pop_front();
        }

        if let Some(time) = front {
            self.internal_queue.push_front(time);
        }

        self.internal_queue.push_back(t);

        self.internal_queue.len() as i32
    }
}
