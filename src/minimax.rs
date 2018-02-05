use board::*;

const MIN: i8 = -((ROWS*COLS) as i8);
const MAX: i8 = (ROWS*COLS) as i8;

pub struct AI;

impl AI {
    pub fn negamax(board: &Board) -> i8 {
        let mut best_score = MIN;
        let moves = board.valid_moves();
        if moves.len() == 0 { return 0 }

        for col in moves {
            let next = board.after_move(col);
            if next.is_win() { return (MAX + 1 - board.moves) / 2 }
            let score = -Self::negamax(&next);
            if score > best_score { best_score = score }
        }
        best_score
    }
}
