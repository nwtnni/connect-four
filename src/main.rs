extern crate minimax; 

use minimax::engine::*;
use minimax::player::*;

pub fn main() {
    let engine = human_vs_cpu(Difficulty::Hard);
    // let engine = human_vs_human();
    engine.run()
}
