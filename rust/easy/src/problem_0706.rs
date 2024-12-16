struct MyHashMap {
    elements: Vec<Option<Vec<(i32, i32)>>>,
    amount: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            elements: vec![None],
            amount: 0,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.put_internal(key, value);
        self.amount += 1;
        self.rehash();
    }

    fn get(&self, key: i32) -> i32 {
        let indices = self.find_double_index(key);
        if indices.0 == -1 || indices.1 == -1 {
            -1
        } else {
            self.elements[indices.0 as usize].as_ref().unwrap()[indices.1 as usize].1
        }
    }

    fn remove(&mut self, key: i32) {
        self.remove_with_collision(key)
    }

    fn rehash(&mut self) {
        let current_len = self.elements.len();
        if 2 * self.amount > self.elements.len() {
            self.elements.resize(2 * current_len, None);
            for i in 0..current_len {
                if let Some(key_values) = self.elements[i].take() {
                    self.elements[i] = None;
                    for key_value in key_values {
                        self.put_internal(key_value.0, key_value.1)
                    }
                }
            }
        }
    }

    fn remove_with_collision(&mut self, key: i32) {
        let indices = self.find_double_index(key);
        if indices.0 == -1 || indices.1 == -1 {
        } else {
            let key_values = self.elements[indices.0 as usize].as_mut().unwrap();
            key_values.remove(indices.1 as usize);
        }
    }

    fn find_double_index(&self, key: i32) -> (i32, i32) {
        let first_index = self.hash(key);
        match &self.elements[first_index] {
            None => (-1, -1),
            Some(collisions) => {
                for (index, col) in collisions.iter().enumerate() {
                    if col.0 == key {
                        return (first_index as i32, index as i32);
                    }
                }
                (first_index as i32, -1)
            }
        }
    }

    fn put_internal(&mut self, key: i32, value: i32) {
        let hash = self.hash(key);

        if let Some(values) = self.elements[hash].as_mut() {
            let mut index = -1;
            for (i, key_value) in values.iter().enumerate() {
                if key_value.0 == key {
                    index = i as i32;
                    break;
                }
            }
            if index == -1 {
                values.push((key, value))
            } else {
                values[index as usize] = (key, value)
            }
        } else {
            self.elements[hash] = Some(vec![(key, value)])
        }
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::problem_0706::MyHashMap;

    #[test]
    fn test_1() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.remove(2);
        assert_eq!(map.get(2), -1);
    }
}
