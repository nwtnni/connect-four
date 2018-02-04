use grid::*;
use minimax::best_move;

const POSITIVE: [i32; 5] = [0, 1, 4, 9, 10000];
const NEGATIVE: [i32; 5] = [0, -1, -8, -27, -1000000];

pub struct Human {}

pub struct CPU {
    pub depth: u8,
    pub positive: [i32; 5],
    pub negative: [i32; 5],
}

pub enum Difficulty { Easy, Medium, Hard }

pub trait Player {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8;
}

impl Player for Human {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8 {
        let mut col: u8 = read!();
        while let None = grid.next(col, color) {
            col = read!();
        }
        col
    }
}

impl Player for CPU {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8 {
        best_move(self, grid, color, self.depth)
    }
}

impl CPU {
    pub fn new(d: Difficulty) -> Self {
        let depth = match d {
            Difficulty::Easy => 2,
            Difficulty::Medium => 5,
            Difficulty::Hard => 11,
        };
        CPU { depth, positive: POSITIVE, negative: NEGATIVE }
    }
}
