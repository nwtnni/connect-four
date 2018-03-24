use board::*;
use table::*;
use opening::*;

pub const SIZE: i8 = (ROWS*COLS) as i8;

pub struct AI {
    table: Table,
    lookup: Lookup,
}

impl AI {
    pub fn new() -> Self {
        AI { table: Table::new(), lookup: Lookup::new() }
    }

    pub fn solve(&mut self, board: &mut Board) -> u8 {
        let safe = board.safe_moves();
        let mut best_score = SIZE;
        let mut best_col = 3;

        for &col in &safe {
            if board.will_win(col) { return col }
        }

        for &col in &safe {

            let score = if board.moves < 6 {
                board.make_move(col);
                let score = self.lookup.get(board.key());
                board.undo_move(col);
                score
            } else {
                self.negamax(board, -1, 1)
            };

            if score < best_score {
                best_score = score;
                best_col = col;
            }
        }
        return best_col
    }

    pub fn negamax(&mut self, board: &mut Board, mut alpha: i8, mut beta: i8) -> i8 {
        let moves = board.safe_moves();

        if moves.len() == 0 { return -1 }
        if board.moves >= SIZE - 2 { return 0 }
        if alpha >= beta { return alpha }

        if let Some(score) = self.table.get(board.key()) {
            if beta > score { beta = score }
        };

        for col in moves {
            board.make_move(col);
            let score = -self.negamax(board, -beta, -alpha);
            board.undo_move(col);

            if score >= beta { return beta }
            if score > alpha { alpha = score }
        }

        self.table.insert(board.key(), alpha);
        return alpha
    }
}
