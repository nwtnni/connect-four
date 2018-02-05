pub const ROWS: u8 = 6;
pub const COLS: u8 = 7;
pub const WHITE: i8 = 0b1;
pub const BLACK: i8 = 0b0;

const TOP_MASK: [u64; 7] = [
    0b0000000_0000000_0000000_0000000_0000000_0000000_0100000,
    0b0000000_0000000_0000000_0000000_0000000_0100000_0000000,
    0b0000000_0000000_0000000_0000000_0100000_0000000_0000000,
    0b0000000_0000000_0000000_0100000_0000000_0000000_0000000,
    0b0000000_0000000_0100000_0000000_0000000_0000000_0000000,
    0b0000000_0100000_0000000_0000000_0000000_0000000_0000000,
    0b0100000_0000000_0000000_0000000_0000000_0000000_0000000,
];

const BOT_MASK: [u64; 7] = [
    0b0000000_0000000_0000000_0000000_0000000_0000000_0000001,
    0b0000000_0000000_0000000_0000000_0000000_0000001_0000000,
    0b0000000_0000000_0000000_0000000_0000001_0000000_0000000,
    0b0000000_0000000_0000000_0000001_0000000_0000000_0000000,
    0b0000000_0000000_0000001_0000000_0000000_0000000_0000000,
    0b0000000_0000001_0000000_0000000_0000000_0000000_0000000,
    0b0000001_0000000_0000000_0000000_0000000_0000000_0000000,
];

const COL_MASK: [u64; 7] = [
    0b0000000_0000000_0000000_0000000_0000000_0000000_0111111,
    0b0000000_0000000_0000000_0000000_0000000_0111111_0000000,
    0b0000000_0000000_0000000_0000000_0111111_0000000_0000000,
    0b0000000_0000000_0000000_0111111_0000000_0000000_0000000,
    0b0000000_0000000_0111111_0000000_0000000_0000000_0000000,
    0b0000000_0111111_0000000_0000000_0000000_0000000_0000000,
    0b0111111_0000000_0000000_0000000_0000000_0000000_0000000,
];

const MOVE_ORDER: [u8; 7] = [3, 2, 4, 1, 5, 0, 6];

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Board {
    pub moves: i8,
    pub owned: u64,
    pub all: u64,
}

impl Board {
    pub fn new() -> Self { Board { moves: 0, owned: 0, all: 0 } }

    pub fn valid_moves(&self) -> Vec<&u8> {
        MOVE_ORDER.iter().filter(|&&col| {
            self.all & TOP_MASK[col as usize] == 0
        }).collect()
    }

    pub fn key(&self) -> u64 {
        self.all + self.owned
    }

    pub fn make_move(&mut self, col: u8) {
        self.owned ^= self.all;
        self.all |= self.all + BOT_MASK[col as usize];
        self.moves += 1;
    }

    pub fn will_win(&self, col: u8) -> bool {
        let col = col as usize;
        let owned = self.owned | ((self.all + BOT_MASK[col]) & COL_MASK[col]);
        Self::is_won(owned)
    }

    pub fn is_won(owned: u64) -> bool {
        let l = owned & (owned >> 6);
        if l & (l >> 12) != 0 { return true }

        let r = owned & (owned >> 8);
        if r & (r >> 16) != 0 { return true }

        let h = owned & (owned >> 7);
        if h & (h >> 14) != 0 { return true }

        let v = owned & (owned >> 1);
        if v & (v >> 2) != 0 { return true }

        false
    }
}
