use board::*;
use table::*;

pub const SIZE: i8 = (ROWS*COLS) as i8;
pub const MIN: i8 = -SIZE/2 + 3;

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

    pub fn solve(&mut self, board: &mut Board) -> u8 {
        let safe = board.safe_moves();
        let mut best_score = SIZE;
        let mut best_col = 3;
        for &col in &safe {
            if board.will_win(col) { return col }
        }
        for &col in &safe {
            let score = self.null_window(board);
            if score < best_score { best_score = score; best_col = col; }
        }
        best_col
    }

    pub fn null_window(&mut self, board: &mut Board) -> i8 {
        let mut min = -(SIZE - board.moves)/2;
        let mut max = (SIZE+1 - board.moves)/2;
        while min < max {
            let mut mid = min + (max - min)/2;
            if mid <= 0 && min/2 < mid { mid = min/2 }
            else if mid >= 0 && max/2 > mid { mid = max/2 }

            let score = self.negamax(board, mid, mid + 1);
            if score <= mid { max = score }
            else { min = score }
        }
        min
    }

    pub fn negamax(&mut self, board: &mut Board, mut alpha: i8, mut beta: i8) -> i8 {
        let moves = board.safe_moves();
        if moves.len() == 0 { return -(SIZE - board.moves)/2 }
        if board.moves >= SIZE - 2 { return 0 }

        let min = -(SIZE - 2 - board.moves) / 2;
        if alpha < min { alpha = min }
        if alpha >= beta { return alpha }

        let max = match self.table.get(board.key()) {
            Some(bound) => bound,
            None        => (SIZE - 1 - board.moves) / 2
        };

        if beta > max { beta = max }
        if alpha >= beta { return beta }

        for col in moves {
            board.make_move(col);
            let score = -self.negamax(board, -beta, -alpha);
            board.undo_move(col);

            if score >= beta { return score }
            if score > alpha { alpha = score }
        }

        self.table.insert(board.key(), alpha);
        alpha
    }
}
