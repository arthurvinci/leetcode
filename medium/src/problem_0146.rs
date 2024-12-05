use std::collections::HashMap;

struct LRUCache {
    map: HashMap<i32, LRUCacheNode>,
    head: Option<i32>,
    tail: Option<i32>,
    capacity: usize,
}

#[derive(Clone)]
struct LRUCacheNode {
    key: i32,
    value: i32,
    prev: Option<i32>,
    next: Option<i32>,
}

impl LRUCacheNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.remove(key) {
            None => -1,
            Some(val) => {
                self.put_first(key, val);
                val
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.remove(key);
        self.put_first(key, value)
    }

    fn put_first(&mut self, key: i32, value: i32) {
        let mut lru_node = LRUCacheNode::new(key, value);
        match &mut self.head {
            None => {
                self.head = Some(key);
                self.tail = Some(key);
            }
            Some(head_key) => {
                let prev_head = self.map.get_mut(head_key).unwrap();
                prev_head.prev = Some(lru_node.key);
                lru_node.next = Some(*head_key);
                *head_key = key;
            }
        }
        self.map.insert(key, lru_node);

        if self.map.len() > self.capacity {
            self.remove(self.tail.unwrap());
        }
    }

    fn remove(&mut self, key: i32) -> Option<i32> {
        let ret = self.map.remove(&key);

        if let Some(node) = &ret {
            if self.tail.unwrap() == key {
                self.tail = node.prev;
            }

            if self.head.unwrap() == key {
                self.head = node.next;
            }

            if let Some(prev_key) = node.prev {
                let prev_node = self.map.get_mut(&prev_key).unwrap();
                prev_node.next = node.next;
            }

            if let Some(next_key) = node.next {
                let next_node = self.map.get_mut(&next_key).unwrap();
                next_node.prev = node.prev;
            }
        }

        ret.map(|node| node.value)
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use crate::problem_0146::LRUCache;

    #[test]
    fn test_1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.get(1);
        cache.put(3, 3);
        cache.get(2);
        cache.put(4, 4);
        cache.get(1);
        cache.get(3);
        cache.get(4);
    }

    #[test]
    fn test_2() {
        let mut cache = LRUCache::new(1);
        cache.put(2, 1);
        cache.get(2);
        cache.put(3, 2);
        cache.get(2);
        cache.get(3);
    }

    #[test]
    fn test_3() {
        let mut cache = LRUCache::new(2);
        cache.put(2, 1);
        cache.put(1, 1);
        cache.put(2, 3);
        cache.put(4, 1);
        cache.get(1);
        cache.get(2);
    }

    #[test]
    fn test_4() {
        let mut cache = LRUCache::new(2);
        cache.put(2, 1);
        cache.put(3, 2);
        cache.get(3);
        cache.get(2);
        cache.put(4, 3);
        cache.get(2);
        cache.get(3);
        cache.get(4);
    }
}
