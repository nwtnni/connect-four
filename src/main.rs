extern crate minimax; 

use minimax::engine::*;
use minimax::player::*;

pub fn main() {
    let engine = cpu_vs_cpu(10, 7);
    // let engine = human_vs_human();
    engine.run()
}
