pub const ROWS: u8 = 6;
pub const COLS: u8 = 7;
pub const WHITE: i8 = 0b1;
pub const BLACK: i8 = 0b0;

const BOARD_MASK: u64 = 0b0111111_0111111_0111111_0111111_0111111_0111111_0111111;
const BOTTOM_ROW: u64 = 0b0000001_0000001_0000001_0000001_0000001_0000001_0000001;

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

    pub fn safe_moves(&self) -> Vec<u8> {
        let safe = self.safe();
        let mut moves = MOVE_ORDER.iter().filter(|&&col| {
            self.all & TOP_MASK[col as usize] == 0
        }).filter(|&&col| {
            safe & COL_MASK[col as usize] != 0
        }).cloned().collect::<Vec<_>>();

        moves.sort_by_key(|&col| {
            let moved = safe & COL_MASK[col as usize];
            self.score_move(moved)
        });
        moves
    }

    pub fn key(&self) -> u64 {
        self.all + self.owned
    }

    pub fn make_move(&mut self, col: u8) {
        self.owned ^= self.all;
        self.all |= self.all + BOT_MASK[col as usize];
        self.moves += 1;
    }

    pub fn undo_move(&mut self, col: u8) {
        self.moves -= 1;
        self.all ^= ((self.all & COL_MASK[col as usize]) + BOT_MASK[col as usize]) >> 1;
        self.owned ^= self.all;
    }

    pub fn will_win(&self, col: u8) -> bool {
        (self.win_positions() & self.possible() & COL_MASK[col as usize]) != 0
    }

    fn score_move(&self, moved: u64) -> i8 {
        -(Self::get_winning_positions(self.owned | moved, self.all | moved)
            .count_ones() as i8)
    }

    fn safe(&self) -> u64 {
        let mut possible = self.possible();
        let opponent = self.opponent_win();
        let forced = possible & opponent;

        if forced != 0 {
            if forced & (forced - 1) != 0 {
                return 0
            } else {
                possible = forced;
            }
        }
        possible & !(opponent >> 1)
    }

    fn win_positions(&self) -> u64 {
        Self::get_winning_positions(self.owned, self.all)
    }

    fn opponent_win(&self) -> u64 {
        Self::get_winning_positions(self.owned ^ self.all, self.all)
    }

    fn possible(&self) -> u64 {
        (self.all + BOTTOM_ROW) & BOARD_MASK
    }

    fn get_winning_positions(owned: u64, all: u64) -> u64 {
        let mut w = (owned << 1) & (owned << 2) & (owned << 3);

        let mut h = (owned << 7) & (owned << 14);
        w |= h & (owned << 21);
        w |= h & (owned >> 7);
        h >>= 21;
        w |= h & (owned << 7);
        w |= h & (owned >> 21);

        let mut l = (owned << 6) & (owned << 12);
        w |= l & (owned << 18);
        w |= l & (owned >> 6);
        l >>= 18;
        w |= l & (owned << 6);
        w |= l & (owned >> 18);

        let mut r = (owned << 8) & (owned << 16);
        w |= r & (owned << 24);
        w |= r & (owned >> 8);
        r >>= 24;
        w |= r & (owned << 8);
        w |= r & (owned >> 24);

        w & (BOARD_MASK ^ all)
    }
}
