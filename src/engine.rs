use fnv::FnvHashSet;
use board::*;
use minimax::*;

pub struct Human;

pub trait Player {
    fn take_turn(&mut self, board: &mut Board) -> u8;
}

impl Player for Human {
    fn take_turn(&mut self, board: &mut Board) -> u8 {
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
    fn take_turn(&mut self, board: &mut Board) -> u8 {
        self.reset();
        self.solve(board)
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
        player_one: Human,
        player_two: Human,
    }
}

pub fn human_vs_cpu() -> Engine<AI, Human> {
    Engine {
        board: Board::new(),
        player_one: AI::new(),
        player_two: Human,
    }
}

pub fn cpu_vs_cpu() -> Engine<AI, AI> {
    Engine {
        board: Board::new(),
        player_one: AI::new(),
        player_two: AI::new(),
    }
}

impl<P1: Player, P2: Player> Engine<P1, P2> {
    pub fn run(mut self) {
        loop {
            println!("{}", self.board);
            if let Some(color) = self.board.was_won() {
                let name = if color == WHITE { "White" } else { "Black" };
                println!("Game over. {} won!", name);
                break
            } else if self.board.valid_moves().len() == 0 {
                println!("Game over. Players tied!");
                break
            } else if self.board.moves & 1 == WHITE {
                let col = self.player_one.take_turn(&mut self.board);
                println!("\nPlayer one played column {}!", col);
                self.board.make_move(col);
            } else {
                let col = self.player_two.take_turn(&mut self.board);
                println!("\nPlayer two played column {}!", col);
                self.board.make_move(col);
            }
        }
    }
}
