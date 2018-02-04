use std::i32::{MIN, MAX};
use player::*;
use grid::*;

const ORDER: [u8; COLS as usize] = [3, 4, 2, 5, 1, 6, 0];

pub struct State {
    pub grid: Grid,
    pub color: Color,
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

pub trait Scorer {
    fn score(&self, state: &State, color: Color) -> i32;
}

impl Scorer for CPU {
    fn score(&self, state: &State, color: Color) -> i32 {
        let mut score = 0;
        if color == Color::W {
            for &(w, b) in &state.grid.counts() {
                score += self.positive[w];
                score += self.negative[b];
            }
        } else {
            for &(w, b) in &state.grid.counts() {
                score += self.positive[b];
                score += self.negative[w];
            }
        }
        score
    }
}

pub fn best_move<S: Scorer>(scorer: &S, grid: &Grid, color: Color, depth: u8) -> u8 {
    let state = State { grid: grid.clone(), color };
    let (mut alpha, beta) = (MIN, MAX);
    let (mut max, mut column) = (MIN, 0);
    for &col in &ORDER {
        if let Some(child) = state.next(col) {
            let score = minimax(scorer, &child, color, depth - 1, alpha, beta);
            if score > alpha { alpha = score }
            if score > max { max = score; column = col }
            if beta <= alpha { break }
        }
    }
    column
}

fn minimax<S: Scorer>(scorer: &S, state: &State, color: Color, depth: u8, alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        scorer.score(state, color)
    } else if state.color == color {
        let mut alpha = alpha;
        let mut max = MIN;
        for child in ORDER.iter().filter_map(|&col| state.next(col)) {
            let score = minimax(scorer, &child, color, depth - 1, alpha, beta);
            if score > alpha { alpha = score }
            if score > max { max = score }
            if beta <= alpha { break }
        }
        max
    } else {
        let mut beta = beta;
        let mut min = MAX;
        for child in ORDER.iter().filter_map(|&col| state.next(col)) {
            let score = minimax(scorer, &child, color, depth - 1, alpha, beta);
            if score < beta { beta = score }
            if score < min { min = score }
            if beta <= alpha { break }
        }
        min
    }
}
