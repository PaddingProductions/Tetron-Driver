mod sandbox;
mod board;

use std::collections::{HashMap, VecDeque};
use tetron::*;
use std::time::{Instant};

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
    
}

#[cfg(test)]
mod test {
    use super::*;
}
