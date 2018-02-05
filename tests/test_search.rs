extern crate minimax;

use std::io::*;
use std::fs::File;
use std::str::FromStr;
use std::time::{Instant, Duration};
use minimax::board::*;
use minimax::minimax::*;

const END_EASY: &'static str = "end-easy.txt";
const MIDDLE_EASY: &'static str = "middle-easy.txt";
const BEGIN_EASY: &'static str = "begin-easy.txt";
const MIDDLE_MEDIUM: &'static str = "middle-medium.txt";
const BEGIN_MEDIUM: &'static str = "begin-medium.txt";
const BEGIN_HARD: &'static str = "begin-hard.txt";

struct Case {
    board: Board,
    score: i8,
}

trait Parse {
    fn parse(moves: &str) -> Self;
}

impl Parse for Board {
    fn parse(moves: &str) -> Self {
        let mut board = Board::new();
        for c in moves.chars() {
            let c = (char::to_digit(c, 10).unwrap() - 1) as u8;
            board = board.after_move(c);
        }
        board
    }
}

fn read_file(file: &'static str) -> BufReader<File> {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/tests/" + file;
    BufReader::new(File::open(path).expect("Could not find file."))
}

fn parse(file: &'static str) -> Vec<Case> {
    let mut cases = Vec::new();
    for line in read_file(file).lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let moves = parts.next().unwrap();
        let board = Board::parse(moves);
        let score = i8::from_str(parts.next().unwrap()).unwrap();
        cases.push(Case { board, score });
    }
    cases
}

fn elapsed(t: Duration) -> f64 {
    let s = t.as_secs() as f64;
    let ns = t.subsec_nanos() as f64;
    s + (ns / 1_000_000_000.0)
}

fn analyze(times: Vec<f64>) -> (f64, f64) {
    let total = times.len() as f64;
    let mean = times.iter().sum::<f64>() / total;
    let std = times.iter().map(|time| {
        let residual = time - mean;
        (residual*residual) / (total - 1.0)
    }).sum::<f64>().powf(0.5);
    (mean, std)
}

fn run_test(file: &'static str) {
    let mut total = 0;
    let mut correct = 0;
    let mut times = Vec::new();
    for case in parse(file) {
        let start = Instant::now();
        let guess = AI::negamax(&case.board);
        let stop = Instant::now();
        if guess == case.score { correct += 1; }
        times.push(elapsed(stop - start));
        total += 1;
        if total % 25 == 0 {
            println!("Current iteration: {}", total);
        }
    }
    let (mean, std) = analyze(times);
    println!("Statistics for {}", file);
    println!("Correctness: {}/{}", correct, total);
    println!("Mean search time: {}", mean);
    println!("Standard deviation: {}", std);
}

#[test]
fn end_easy() {
    run_test(END_EASY);
}

// #[test]
// fn middle_easy() {
//     run_test(MIDDLE_EASY);
// }
