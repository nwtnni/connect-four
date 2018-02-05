const TABLE_SIZE: usize = 8388593;

#[derive(Copy, Clone)]
struct Entry {
    key: u64,
    bound: i8,
}

pub struct Table { map: Vec<Entry>, }

impl Table {
    pub fn new() -> Self {
        Table { map: vec![Entry { key: 0, bound: 0 }; TABLE_SIZE] }
    }

    pub fn reset(&mut self) {
        for entry in self.map.iter_mut() {
            entry.key = 0; 
        } 
    }

    pub fn index(key: u64) -> usize {
        key as usize % TABLE_SIZE
    }

    pub fn insert(&mut self, key: u64, bound: i8) {
        let mut entry = &mut self.map[Self::index(key)];
        entry.key = key;
        entry.bound = bound;
    }

    pub fn get(&self, key: u64) -> Option<i8> {
        let entry = self.map[Self::index(key)];
        if entry.key == key { Some(entry.bound) } else { None }
    }
}
