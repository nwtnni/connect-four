use std::i32::{MIN, MAX};
use std::time::{Instant, Duration};
use fnv::FnvHashMap;
use grid::*;

const ORDER: [u8; COLS as usize] = [3, 4, 2, 5, 1, 6, 0];
const POSITIVE: [i32; 5] = [0, 1, 4, 9, 10000];
const NEGATIVE: [i32; 5] = [0, -1, -8, -27, -100000];

#[derive(Eq, PartialEq, Hash, Clone)]
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

enum Bound { Upper(i32), Lower(i32), Exact(i32) }

impl Bound {
    fn value(&self) -> i32 {
        match *self { Bound::Upper(n) | Bound::Exact(n) | Bound::Lower(n) => n }
    }
}

struct Entry {
    pub bound: Bound,
    pub depth: u8,
    pub best: u8,
}

pub struct AI {
    table: FnvHashMap<State, Entry>,
    timeout: Duration,
    positive: [i32; 5],
    negative: [i32; 5],
}

impl AI {
    pub fn new(t: u64) -> Self {
        let table = FnvHashMap::with_capacity_and_hasher(1_000_000, Default::default());
        let timeout = Duration::from_millis(t);
        let positive = POSITIVE.clone();
        let negative = NEGATIVE.clone();
        AI { table, timeout, positive, negative }
    }

    fn score(&self, state: &State) -> i32 {
        let mut score = 0;
        if state.color == Color::W {
            for &(w, b) in &state.grid.counts() {
                score += self.positive[w];
                score -= self.negative[b];
            }
        } else {
            for &(w, b) in &state.grid.counts() {
                score += self.positive[b];
                score -= self.negative[w];
            }
        }
        score
    }

    pub fn best_move(&mut self, grid: &Grid, color: Color) -> u8 {
        let state = State { grid: grid.clone(), color };
        let (mut min, mut column) = (MAX, 0);
        for &col in &ORDER {
            if let Some(child) = state.next(col) {
                let score = self.ida(&child);
                if score < min { min = score; column = col }
            }
        }
        column
    }

    fn ida(&mut self, state: &State) -> i32 {
        let mut guess = 0;
        let mut depth = 0;
        let instant = Instant::now();
        loop {
            depth += 1;
            guess = self.mtdf(state, guess, depth);
            if instant.elapsed() > self.timeout { println!("Reached depth: {}", depth); break }
        }
        guess
    }

    fn mtdf(&mut self, state: &State, guess: i32, depth: u8) -> i32 {
        let mut max_score = guess;
        let (mut lower, mut upper) = (MIN, MAX);
        while lower < upper {
            let beta = if max_score == lower { max_score + 1 } else { max_score };
            max_score = self.negamax(state, depth, beta - 1, beta);
            if max_score < beta { upper = max_score } else { lower = max_score }
        }
        max_score
    }

    fn negamax(&mut self, state: &State, depth: u8, alpha: i32, beta: i32) -> i32 {
        let mut order = ORDER.iter().cloned().collect::<Vec<_>>();
        let mut alpha = alpha;
        let mut beta = beta;
        if let Some(entry) = self.table.get(state) {
            if entry.depth >= depth {
                match entry.bound {
                    Bound::Upper(score) => if score < beta { beta = score },
                    Bound::Exact(score) => return score,
                    Bound::Lower(score) => if score > alpha { alpha = score },
                }
                if alpha >= beta { return entry.bound.value() }
            }
            order.retain(|&col| col != entry.best);
            order.insert(0, entry.best);
        }

        if depth == 0 {
            self.score(state)
        } else {
            let mut max_score = MIN;
            let mut best_move = 3;
            for &col in &order {
                if let Some(child) = state.next(col) {
                    let score = -self.negamax(&child, depth - 1, -beta, -alpha);
                    if score > max_score { best_move = col; max_score = score }
                    if max_score > alpha { alpha = max_score }
                    if max_score >= beta { best_move = col; break }
                }
            }

            if max_score == MIN { return self.score(state) }

            let entry = if max_score <= alpha {
                Entry { bound: Bound::Lower(max_score), depth, best: best_move }
            } else if max_score >= beta {
                Entry { bound: Bound::Upper(max_score), depth, best: best_move }
            } else {
                Entry { bound: Bound::Exact(max_score), depth, best: best_move }
            };

            self.table.insert(state.clone(), entry);
            max_score
        }
    }
}
