mod sandbox;
mod board;
mod exams;

use tetron::*;

use std::collections::{VecDeque};
use std::io::{self};
use std::time::Instant;

pub use board::Board;

pub mod colors {
    pub const RST: &str = "\x1b[0m";
    pub const BLD: &str = "\x1b[1m";
    pub const HLT: &str = "\x1b[48;5;226m";
    macro_rules! piece_color {
        ($p: expr) => {
            match $p {
                Piece::None => "\x1b[47;1m", // white
                Piece::J => "\x1b[48;5;208m", // bright red / orange
                Piece::L => "\x1b[48;5;20m", // blue
                Piece::S => "\x1b[48;5;9m", // red
                Piece::Z => "\x1b[48;5;46m", // green
                Piece::T => "\x1b[45;1m", // magenta
                Piece::I => "\x1b[48;5;51m", // cyan
                Piece::O => "\x1b[48;5;226m", // yellow
            }
        };
    }
    macro_rules! color {
        (white)   => {"\x1b[47;1m"}; // white
        (orange)  => {"\x1b[41;1m"}; // bright red / orange
        (blue)    => {"\x1b[44;1m"}; // blue
        (red)     => {"\x1b[41;1m"}; // red
        (green)   => {"\x1b[42;1m"}; // green
        (magenta) => {"\x1b[45;1m"}; // magenta
        (cyan)    => {"\x1b[46;1m"}; // cyan
        (yellow)  => {"\x1b[43;1m"}; // yellow
    }
    pub(crate) use color;
    pub(crate) use piece_color;
}
use crate::colors::*;

/*
    Benchmarking function, for tests.
 */
fn bench (iter: u8, f: fn()) -> u128 {
    let mut avg_dt: u128 = 0;
    for _ in 0..iter {
        let start = Instant::now();
        f();
        let dt = start.elapsed().as_micros();
        avg_dt = if avg_dt == 0 {dt} else {(avg_dt + dt) / 2}
    }
    avg_dt
}

fn gen_moves_dummy_fn () {
    let mut state: State = State::new();
    state.pieces = VecDeque::new();
    state.pieces.push_back(Piece::L);
    state.pieces.push_back(Piece::T);

    let map = gen_moves(&state);
    let sum: u32 = map.iter().map(|(field, mov)| mov.x as u32).sum::<u32>();
    print!("{}", sum);
}

fn main() {
    println!("{HLT}==={{ Tetron CLI }}==={RST}");
    println!("\n{BLD}Commands:{RST}");
    println!("- sandbox [mode, opts: atk, ds, norm | norm]");
    println!("- cheese_exam [iter, num | 10] [lines, num | 18] [log, opts: log]");

    let mut buf = String::new();
    loop {
        io::stdin().read_line(&mut buf).expect("Error on STDIN.");
        buf.pop();

        let args: Vec<&str> = buf.split(" ").collect();
        
        println!("command: {buf}. args: {:?}", args);
        match args[0] {
            "sandbox" => {
                let mode = if args.len() < 2 {EvaluatorMode::Norm} else {match args[1] {
                    "atk" => EvaluatorMode::Attack,
                    "ds" => EvaluatorMode::DS,
                    "norm" => EvaluatorMode::Norm,
                    _ => EvaluatorMode::Norm,
                }};
                sandbox::run(Some(mode));
            },
            "cheese_exam" => {
                let iter = if args.len() > 1 {args[1].parse().unwrap()} else {10};
                let lines = if args.len() > 2 {args[2].parse().unwrap()} else {18};
                let log = if args.len() > 3 {args[3] == "log"} else {false};
                exams::cheese_exam(iter, lines, log);
            }
            _ => continue
        }
        break;
    }
}