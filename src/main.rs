mod sandbox;
mod board;

use tetron::*;

use std::collections::{VecDeque};
use std::time::Instant;

pub mod colors {
    pub const RST: &str = "\x1b[0m";
    pub const BLD: &str = "\x1b[1m";

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
    println!("{}", sum);
}

fn main() {
    println!("\x1b[43;1mStarting sandbox..\x1b[0m");
    sandbox::run();    
    //let out = sandbox_bench_fn();
    //println!("sandbox bench avg_dt: {BLD}{}{RST}", out);
}