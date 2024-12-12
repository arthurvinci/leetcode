use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    indices: HashMap<i32, usize>,
    elements: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            indices: HashMap::new(),
            elements: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            false
        } else {
            self.indices.insert(val, self.elements.len());
            self.elements.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.indices.remove(&val) {
            None => false,
            Some(index) => {
                self.elements[index] = *self.elements.last().unwrap();
                self.indices
                    .get_mut(&self.elements[index])
                    .map(|old_index| *old_index = index);

                self.elements.pop();
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::rng();
        let random_idx = rng.random_range(0..self.elements.len());
        self.elements[random_idx]
    }
}

#[cfg(test)]
mod test {
    use crate::problem_0380::RandomizedSet;

    #[test]
    fn test_1() {
        let mut set = RandomizedSet::new();
        set.insert(0);
        set.insert(1);
        set.remove(0);
        set.insert(2);
        set.remove(1);
        set.get_random();
    }
}
