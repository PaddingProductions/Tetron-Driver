use tetron::{solve, Piece, State, EvaluatorMode};

use std::time::Instant;
use std::{thread, time::Duration};
use crate::board::*;
use crate::colors::*;


pub fn sandbox_bench_fn () -> u128 {
    const ITERS: i32 = 100;

    let mut state = State::new();
    let mut bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O];
    
    let mut avg_dt = 0;
    while state.pieces.len() < 6 {
        state.pieces.push_back(super::draw(&mut bag));
    }
    for _ in 0..ITERS {
        // Draw pieces
        while state.pieces.len() < 6 {
            state.pieces.push_back(super::draw(&mut bag));
        }

        // Solve & Bench
        let start = Instant::now();        
        if let Some(out) = solve(&state, 1, None) {
            state = out.0;
        } else {
            state = State::new();
        }
        let dt = start.elapsed().as_micros();
        avg_dt = if avg_dt == 0 {dt} else {(avg_dt + dt) / 2};
    }
    avg_dt
}

pub fn sandbox_run (iters: u32, mode: Option<EvaluatorMode>) {
    const SANDBOX_DELAY: u64 = 0;

    let mut state = State::new();
    println!("{BLD}=== Sandbox Run ==={RST}");
    println!("mode:  {:?}", mode);
    println!("iters: {:?}", iters);

    let mut board = Board::new(Some(&state.field));
    let mut bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O];
    
    let mut atks: u32 = 0;
    let mut total_dt = 0.0;
    let mut avg_dt = 0;
    while state.pieces.len() < 6 {
        state.pieces.push_back(super::draw(&mut bag));
    }
    println!("init state:\n{}", state);
    for _ in 0..iters {
        // Draw pieces
        while state.pieces.len() < 6 {
            state.pieces.push_back(super::draw(&mut bag));
        }

        // Solve & Bench
        tetron::bench_increment_solve();
        let start = Instant::now();        
        if let Some(out) = solve(&state, 3, mode) {
            let dt = start.elapsed().as_micros();
            total_dt += dt as f64 / 1_000_000.0;
            avg_dt = if avg_dt == 0 {dt} else {(avg_dt + dt) / 2};

            println!("Time consumed: {}{}{}us", BLD, dt, RST);
            println!("Avg benchmark: {}{}{}us", BLD, avg_dt, RST);

            // Attack tracing
            atks += out.0.props.atk as u32;

            // Apply move to colored board
            board.apply_move(&out.1, &state.pieces[0], if state.hold == Piece::None {&state.pieces[1]} else {&state.hold});

            // Log out result
            println!("result score: {BLD}{}{RST}", out.2);
            super::render(&board, &out.0);


            state.field = out.0.field;
            state.pieces = out.0.pieces;
            state.hold = out.0.hold;
        } else {
            println!("{BLD}No results found, game over.{RST}");
            break;
        }
        thread::sleep(Duration::from_millis(SANDBOX_DELAY));
    }
    println!("=== Results ==="); 
    println!("apm: {:.2}, attacks: {}", atks as f64 / (total_dt as f64 / 60.0), atks);
    println!("pps: {:.2}, pieces: {}", (iters as f64 / total_dt as f64), iters);

    tetron::print_bench_result();
}

