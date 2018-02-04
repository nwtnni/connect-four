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
    pub fn run(self) {
            

    
    
    
    }
}
