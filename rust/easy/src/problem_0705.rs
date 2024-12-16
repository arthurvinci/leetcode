struct MyHashSet {
    elements: Vec<Option<Vec<i32>>>,
    amount: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl MyHashSet {
    fn new() -> Self {
        Self {
            elements: vec![None],
            amount: 0,
        }
    }

    fn add(&mut self, key: i32) {
        self.add_with_collision(key);
        self.rehash();
        self.amount += 1;
    }

    fn remove(&mut self, key: i32) {
        let double_index = self.find_double_index(key);

        if double_index.0 != -1 && double_index.1 != -1 {
            let keys = self.elements[double_index.0 as usize].as_mut().unwrap();
            keys.remove(double_index.1 as usize);
        }
    }

    fn contains(&self, key: i32) -> bool {
        let double_index = self.find_double_index(key);

        double_index.0 != -1 && double_index.1 != -1
    }

    fn add_with_collision(&mut self, key: i32) {
        let hash = self.hash(key);
        if let Some(keys) = self.elements[hash].as_mut() {
            let mut index = -1;

            for (i, cur_key) in keys.iter().enumerate() {
                if key == *cur_key {
                    index = i as i32;
                    break;
                }
            }

            if index == -1 {
                keys.push(key)
            }
        } else {
            self.elements[hash] = Some(vec![key]);
        }
    }

    fn rehash(&mut self) {
        if 2 * self.amount > self.elements.len() {
            let cur_len = self.elements.len();
            self.elements.resize(2 * self.elements.len(), None);

            for i in 0..cur_len {
                if let Some(keys) = self.elements[i].take() {
                    self.elements[i] = None;
                    for key in keys {
                        self.add_with_collision(key);
                    }
                }
            }
        }
    }

    fn find_double_index(&self, key: i32) -> (i32, i32) {
        let first_index = self.hash(key);
        match &self.elements[first_index] {
            None => (-1, -1),
            Some(collisions) => {
                for (index, cur_key) in collisions.iter().enumerate() {
                    if *cur_key == key {
                        return (first_index as i32, index as i32);
                    }
                }
                (first_index as i32, -1)
            }
        }
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % self.elements.len()
    }
}
