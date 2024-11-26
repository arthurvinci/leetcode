use std::collections::HashMap;

struct LRUCacheNode {
    key: i32,
    value: i32,
    prev_key: Option<i32>,
    next_key: Option<i32>,
}

struct LRUCache {
    capacity: usize,
    starting_node: Option<i32>,
    ending_node: Option<i32>,
    elements: HashMap<i32, LRUCacheNode>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            starting_node: None,
            ending_node: None,
            elements: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let mut previous_starting_node = None;
        let mut previous_node = None;
        let mut next_node = None;
        let mut node_value = -1;

        match self.elements.get_mut(&key) {
            None => {}
            Some(node) => {
                previous_node = node.prev_key;
                next_node = node.next_key;

                if self.starting_node != Some(key) {
                    previous_starting_node = self.starting_node;
                }

                if let Some(ending_node) = self.ending_node {
                    if ending_node == key && previous_node.is_some() {
                        self.ending_node = node.prev_key;
                    }
                }

                if node.next_key.is_some() {
                    node.next_key = self.starting_node;
                }
                node.prev_key = None;
                self.starting_node = Some(node.key);

                node_value = node.value;
            }
        }

        self.update_previous_and_next(previous_node, next_node);

        if let Some(starting_node) = previous_starting_node {
            let node = self.elements.get_mut(&starting_node).unwrap();
            node.prev_key = Some(key)
        }

        node_value
    }

    fn update_previous_and_next(&mut self, previous_node: Option<i32>, next_node: Option<i32>) {
        if previous_node.is_some() {
            let node = self.elements.get_mut(&previous_node.unwrap()).unwrap();
            node.next_key = next_node;

            if let Some(next_node) = next_node {
                let node = self.elements.get_mut(&next_node).unwrap();
                node.prev_key = previous_node;
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let mut previous_node = None;
        let mut next_node = None;

        if let Some(node) = self.elements.get_mut(&key) {
            node.value = value;
            previous_node = node.prev_key;
            next_node = node.next_key;
            if node.prev_key.is_some() {
                self.ending_node = node.prev_key;
            }
        } else {
            let lru_node = LRUCacheNode {
                key,
                value,
                prev_key: None,
                next_key: self.starting_node,
            };

            self.elements.insert(key, lru_node);
        }

        self.update_previous_and_next(previous_node, next_node);

        if let Some(starting_node) = self.starting_node {
            let elem = self.elements.get_mut(&starting_node).unwrap();
            elem.prev_key = Some(key);
        }

        self.starting_node = Some(key);

        if self.elements.len() == 1 {
            self.ending_node = Some(key);
        }

        if self.elements.len() > self.capacity {
            let last_node = self.elements.remove(&self.ending_node.unwrap());
            self.ending_node = last_node.unwrap().prev_key;

            if let Some(end_node) = self.ending_node {
                let node = self.elements.get_mut(&end_node).unwrap();
                node.next_key = None;
            }
        }
    }
}

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
