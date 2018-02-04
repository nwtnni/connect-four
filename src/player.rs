use grid::*;
use minimax::best_move;

pub trait Player {
    fn take_turn(&self, grid: &Grid, color: Color) -> u8;
}

pub struct Human {}

pub struct CPU {
    depth: u8,
    score: [i32; 10],
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
