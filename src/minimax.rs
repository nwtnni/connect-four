use board::*;

pub const MIN: i8 = -((ROWS*COLS) as i8);
pub const MAX: i8 = (ROWS*COLS) as i8;

pub struct AI;

impl AI {
    pub fn negamax(board: &Board, mut alpha: i8, mut beta: i8) -> i8 {
        let moves = board.valid_moves();
        if moves.len() == 0 { return 0 }

        let max = (MAX + 1 - board.moves) / 2;
        if beta > max { beta = max }
        if alpha >= beta { return beta }

        for &col in moves {
            let next = board.after_move(col);
            if next.is_win() { return max }
            let score = -Self::negamax(&next, -beta, -alpha);

            if score >= beta { return score }
            if score > alpha { alpha = score }
        }
        alpha
    }
}
