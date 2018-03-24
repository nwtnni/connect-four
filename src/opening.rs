use std::str::FromStr;
use fnv::FnvHashMap;

use board::*;

pub struct Lookup {
    table: FnvHashMap<u64, i8>
}

impl Lookup {
    pub fn new() -> Self {
        let mut table = FnvHashMap::default();
        for line in include_str!("opening.dat").trim_right().split("\n") {
            let mut parts = line.split_whitespace();
            let board = Board::from(parts.next().unwrap());
            let score = i8::from_str(parts.next().unwrap()).unwrap();
            table.insert(board.key(), score);
        }
        Lookup { table }
    }

    pub fn get(&self, key: u64) -> i8 {
        self.table[&key]
    }
}
