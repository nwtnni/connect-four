use fnv::FnvHashSet;
use board::*;
use minimax::*;

pub struct Human {}

pub trait Player {
    fn take_turn(&mut self, board: &Board) -> u8;
}

impl Player for Human {
    fn take_turn(&mut self, board: &Board) -> u8 {
        let mut col: u8 = read!();
        let valid = board.valid_moves()
            .into_iter()
            .collect::<FnvHashSet<_>>();
        while !valid.contains(&col) {
            col = read!();
        }
        col
    }
}

impl Player for AI {
    fn take_turn(&mut self, board: &Board) -> u8 {
        self.best_move(board)
    }
}

pub struct Engine<P1: Player, P2: Player> {
    board: Board,
    player_one: P1,
    player_two: P2,
}

pub fn human_vs_human() -> Engine<Human, Human> {
    Engine {
        board: Board::new(),
        player_one: Human {},
        player_two: Human {},
    }
}

pub fn human_vs_ai(t: u64) -> Engine<Human, AI> {
    Engine {
        board: Board::new(),
        player_one: Human {},
        player_two: AI::new(t),
    }
}

pub fn ai_vs_ai(t1: u64, t2: u64) -> Engine<AI, AI> {
    Engine {
        board: Board::new(),
        player_one: AI::new(t1),
        player_two: AI::new(t2),
    }
}

impl<P1: Player, P2: Player> Engine<P1, P2> {
    pub fn run(mut self) {
        loop {
            println!("{}", self.board);
            if self.board.is_win() {
                let winner = if self.board.current() == WHITE {
                    "Black"
                } else {
                    "White"
                };
                println!("Game over. {} won!", winner);
                break
            } else if self.board.valid_moves().len() == 0 {
                println!("Game over. Players tied!");
                break
            } else if self.board.current() == WHITE {
                let col = self.player_one.take_turn(&self.board);
                println!("\nPlayer one played column {}!", col);
                self.board = self.board.after_move(col);
            } else {
                let col = self.player_two.take_turn(&self.board);
                println!("\nPlayer two played column {}!", col);
                self.board = self.board.after_move(col);
            }
        }
    }
}
