use grid::*;
use minimax::*;

pub struct Human {}

pub trait Player {
    fn take_turn(&mut self, grid: &Grid, color: Color) -> u8;
}

impl Player for Human {
    fn take_turn(&mut self, grid: &Grid, color: Color) -> u8 {
        let mut col: u8 = read!();
        while let None = grid.next(col, color) {
            col = read!();
        }
        col
    }
}

impl Player for AI {
    fn take_turn(&mut self, grid: &Grid, color: Color) -> u8 {
        self.best_move(grid, color)
    }
}

pub struct Engine<P1: Player, P2: Player> {
    grid: Grid,
    color: Color,
    player_one: P1,
    player_two: P2,
}

pub fn human_vs_human() -> Engine<Human, Human> {
    Engine {
        grid: Grid::new(),
        color: Color::W,
        player_one: Human {},
        player_two: Human {},
    }
}

pub fn human_vs_ai(t: u64) -> Engine<Human, AI> {
    Engine {
        grid: Grid::new(),
        color: Color::W,
        player_one: Human {},
        player_two: AI::new(t),
    }
}

pub fn ai_vs_ai(t1: u64, t2: u64) -> Engine<AI, AI> {
    Engine {
        grid: Grid::new(),
        color: Color::W,
        player_one: AI::new(t1),
        player_two: AI::new(t2),
    }
}

impl<P1: Player, P2: Player> Engine<P1, P2> {
    pub fn run(mut self) {
        loop {
            println!("{}", self.grid);
            if let Some(color) = self.grid.is_winner() {
                println!("Game over. {} won!", color);
                break
            } else if self.grid.is_full() {
                println!("Game over. Players tied!");
                break
            } else if self.color == Color::W {
                let col = self.player_one.take_turn(&self.grid, self.color);
                println!("\nPlayer one played column {}!", col);
                self.grid = self.grid.next(col, self.color).expect("Invalid move by P1");
                self.color = Color::B;
            } else {
                let col = self.player_two.take_turn(&self.grid, self.color);
                println!("\nPlayer two played column {}!", col);
                self.grid = self.grid.next(col, self.color).expect("Invalid move by P2");
                self.color = Color::W;
            }
        }
    }
}
