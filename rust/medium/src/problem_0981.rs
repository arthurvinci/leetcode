use std::cmp::Ordering;
use std::collections::HashMap;

struct TimeMap {
    inner_map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            inner_map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let values = self.inner_map.entry(key).or_insert(vec![]);
        values.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.inner_map.get(&key) {
            None => String::new(),
            Some(values) => {
                let mut left_index = 0;
                let mut right_index = values.len();

                while left_index < right_index {
                    let mid = (right_index + left_index) / 2;
                    match values[mid].0.cmp(&timestamp) {
                        Ordering::Less => {
                            left_index = mid + 1;
                        }
                        Ordering::Equal => {
                            left_index = mid + 1;
                        }
                        Ordering::Greater => {
                            right_index = mid;
                        }
                    }
                }
                if left_index > 0 {
                    values[left_index - 1].1.clone()
                } else {
                    String::new()
                }
            }
        }
    }
}
