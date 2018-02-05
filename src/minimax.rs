use board::*;
use table::*;

pub const SIZE: i8 = (ROWS*COLS) as i8;
pub const MIN: i8 = -SIZE/2 + 3;
pub const MAX: i8 = (SIZE+1)/2 - 3;

pub struct AI {
    table: Table,
}

impl AI {
    pub fn new() -> Self {
        AI { table: Table::new() }
    }

    pub fn reset(&mut self) {
        self.table.reset()
    }

    pub fn negamax(&mut self, board: &Board, mut alpha: i8, mut beta: i8) -> i8 {
        let moves = board.valid_moves();
        if moves.len() == 0 { return 0 }

        for &&col in &moves {
            if board.will_win(col) {
                return (SIZE + 1 - board.moves)/2
            }
        }

        let max = if let Some(bound) = self.table.get(board.key()) {
            bound + MIN - 1
        } else {
            (SIZE - 1 - board.moves) / 2
        };

        if beta > max { beta = max }
        if alpha >= beta { return beta }

        for &col in moves {
            let mut next = board.clone();
            next.make_move(col);
            let score = -self.negamax(&next, -beta, -alpha);

            if score >= beta { return score }
            if score > alpha { alpha = score }
        }

        self.table.insert(board.key(), alpha - MIN + 1);
        alpha
    }
}
