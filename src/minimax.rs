use std::i32::{MIN, MAX};
use grid::*;

const ORDER: [u8; COLS as usize] = [3, 4, 2, 5, 1, 6, 0];

pub struct State {
    grid: Grid,
    color: Color,
}

impl State {
    pub fn next(&self, col: u8) -> Option<Self> {
        self.grid.next(col, self.color).map(|grid| {
            let color = match self.color {
                Color::W => Color::B,
                Color::B => Color::W,
            };
            State { grid, color }
        })
    }
}

pub trait Player {
    fn take_turn(&self, state: &State, color: Color) -> u8;
}

pub trait Scorer {
    fn score(&self, state: &State, color: Color) -> i32;
}

pub fn select_move<P: Player>(state: &State, color: Color, player: &P) -> u8 {
    player.take_turn(state, color)
}

pub fn minimax<S: Scorer>(scorer: &S, state: &State, color: Color, depth: u8, alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        scorer.score(state, color)
    } else if state.color == color {
        let mut alpha = alpha;
        let mut max = MIN;
        for child in ORDER.iter().filter_map(|&col| state.next(col)) {
            let score = minimax(scorer, &child, color, depth - 1, alpha, beta);
            alpha = if score > alpha { score } else { alpha };
            max = if score > max { score } else { max };
            if beta <= alpha { break }
        }
        max
    } else {
        let mut beta = beta;
        let mut min = MAX;
        for child in ORDER.iter().filter_map(|&col| state.next(col)) {
            let score = minimax(scorer, &child, color, depth - 1, alpha, beta);
            beta = if score < beta { score } else { beta };
            min = if score < min { score } else { min };
            if beta <= alpha { break }
        }
        min
    }
}
