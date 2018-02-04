use grid::*;
use player::*;

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

pub fn human_vs_cpu(d: Difficulty) -> Engine<Human, CPU> {
    Engine {
        grid: Grid::new(),
        color: Color::W,
        player_one: Human {},
        player_two: CPU::new(d),
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
