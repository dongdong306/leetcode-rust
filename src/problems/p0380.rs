use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;

struct RandomizedSet {
    array: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        return RandomizedSet{
            array: Vec::new(),
            map: HashMap::new(),
        };
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            false
        } else {
            self.map.insert(val, self.array.len());
            self.array.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            Some(index) => {
                let last_index = self.array.len() - 1;
                self.array.swap_remove(index);
                if last_index != index {
                    let last_value = self.array[index];
                    self.map.insert(last_value, index);
                }

                true
            }
            None => false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.array.choose(&mut rng).expect("RandomizedSet is empty")
    }
}
