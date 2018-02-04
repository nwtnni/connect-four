use std::io;
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

pub trait Player {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8;
    fn wait(&self);
}

impl Player for Human {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8 {
        let mut col: u8 = read!();
        while let None = grid.next(col, color) {
            col = read!();
        }
        col
    }

    fn wait(&self) {}
}

impl Player for CPU {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8 {
        best_move(self, grid, color, self.depth)
    }

    fn wait(&self) { 
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Error in user prompt");
    }
}

impl CPU {
    pub fn new(depth: u8) -> Self {
        CPU { depth, positive: POSITIVE, negative: NEGATIVE }
    }
}
