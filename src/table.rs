const TABLE_SIZE: usize = 8388593;

#[derive(Copy, Clone)]
struct Entry {
    key: u64,
    score: i8,
}

pub struct Table { map: Vec<Entry> }

impl Table {
    pub fn new() -> Self {
        Table {
            map: vec![Entry { key: 0, score: 0 }; TABLE_SIZE],
        }
    }

    pub fn index(key: u64) -> usize {
        key as usize % TABLE_SIZE
    }

    pub fn insert(&mut self, key: u64, score: i8) {
        let entry = &mut self.map[Self::index(key)];
        entry.key = key;
        entry.score = score;
    }

    pub fn get(&mut self, key: u64) -> Option<i8> {
        let entry = self.map[Self::index(key)];
        if entry.key == key {
            Some(entry.score)
        } else {
            None
        }
    }
}
