use std::fmt;

pub const ROWS: u8 = 6;
pub const COLS: u8 = 7;
pub const WHITE: u8 = 0b1;
pub const BLACK: u8 = 0b0;

const DIRECTIONS: [u8; 4] = [1, 6, 7, 8];
const MAX_HEIGHT: u64 = 0x80808080;
const HEIGHT: [u8; COLS as usize] = [
    0, 7, 14, 21, 28, 35, 42
];

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Board {
    turn: u8,
    height: [u8; COLS as usize],
    board: [u64; 2],
}

impl Board {
    pub fn new() -> Self {
        let turn = 0;
        let height = HEIGHT.clone();
        let board = [0, 0];
        Board { turn, height, board }
    }

    pub fn valid_moves(&self) -> Vec<u8> {
        (0..COLS).filter(|&col| {
          MAX_HEIGHT & (1 << self.height[col as usize]) as u64 == 0
        }).collect()
    }

    pub fn after_move(&self, col: u8) -> Self {
        let Board { mut turn, mut height, mut board } = self.clone();
        board[(turn & 1) as usize] ^= 1 << height[col as usize];
        height[col as usize] += 1;
        turn += 1;
        Board { turn, height, board }
    }

    pub fn before_move(&self, col: u8) -> Self {
        let Board { mut turn, mut height, mut board } = self.clone();
        turn -= 1;
        height[col as usize] -= 1;
        board[(turn & 1) as usize] ^= 1 << height[col as usize];
        Board { turn, height, board }
    }

    pub fn is_win(&self) -> bool {
        let board = self.board[(!self.turn & 1) as usize];
        for &direction in &DIRECTIONS {
            let half = board & (board >> direction);
            if half & (half >> direction*2) != 0 { return true }
        }
        false
    }

    pub fn current(&self) -> u8 {
        self.turn & 1 
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in (0..ROWS).rev() {
            let mut bit = row;
            while bit < (ROWS + 1)*COLS {
                let mask = (1 << bit) as u64;
                if self.board[0] != 0 {
                    write!(f, " W");
                } else if self.board[1] != 0 {
                    write!(f, " B");
                } else {
                    write!(f, " .");
                };
                bit += ROWS + 1;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
