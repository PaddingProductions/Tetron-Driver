use tetron::{solve, Piece, State, EvaluatorMode, config};
use std::time::Instant;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::board::Board;
use crate::colors::*;

fn run (pieces: usize, log: bool) -> (u32, f32) {
    let mut state = State::new();
    let mut board = Board::new(Some(&state.field));
    let mut rng   = ChaCha8Rng::seed_from_u64(2);

    let mut total_atk: u32 = 0;

    let mut bag = vec![];
    
    let mut avg_dt: u128 = 0;
    let mut total_dt: f32 = 0.0;
    while state.pieces.len() < 6 {
        state.pieces.push_back(super::draw(&mut rng, &mut bag));
    }
    println!("{}", board);
    for i in 0..pieces {
        // Draw pieces
        while state.pieces.len() < 6 {
            state.pieces.push_back(super::draw(&mut rng, &mut bag));
        }
        
        // Solve & Bench
        let start = Instant::now();        
        if let Some(out) = solve(&state, &config::Config::new(3, EvaluatorMode::Norm)) {
            // Benching
            let dt = start.elapsed().as_micros();
            total_dt += dt as f32 / 1_000_000.0;
            avg_dt = (i as u128 * avg_dt + dt) / (i + 1) as u128;

            // Apply move to colored board
            board.apply_move(
                &out.1, &state.pieces[0], 
                if state.hold == Piece::None {&state.pieces[1]} else {&state.hold}
            );

            // Log out result
            if log {
                println!("Time consumed: {}{}{}us", BLD, dt, RST);
                println!("Avg benchmark: {}{}{}us", BLD, avg_dt, RST);
                super::render(&board, &out.0);
            }
            
            // Add to counters
            total_atk += out.0.props.atk as u32;

            state.field = out.0.field;
            state.pieces = out.0.pieces;
            state.hold = out.0.hold;

        } else {
            println!("{BLD}No results found, game over.{RST}");
            break;
        }
    }
    println!();
    (total_atk, total_dt)
}

pub fn attack_exam (iter: usize, pieces: usize, log: bool) {
    println!("{HLT}--ATTACK EXAM--{RST}");
    println!("{BLD}--iters: {iter}, pieces: {pieces}--{RST}");

    // Results: (average, maximum, minimum)
    let mut apm_res: (f64, f64, f64) = (0.0, 0.0, f64::MAX);
    let mut atk_res: (f64, u32, u32) = (0.0, 0, u32::MAX);
    let mut time_res: (f32, f32, f32) = (0.0, 0.0, f32::MAX);
    let mut avg_pps: f64 = 0.0;

    // Calc & Store Results after each iteration
    for _ in 0..iter {
        let (atk, total_dt) = run(pieces, log);
        let apm: f64 = atk as f64 / (total_dt / 60.0) as f64;
        let pps: f64 = pieces as f64 / total_dt as f64;

        apm_res.0 += apm;
        apm_res.1 =  apm_res.1.max(apm);
        apm_res.2 =  apm_res.2.min(apm);

        atk_res.0 += atk as f64;
        atk_res.1 =  atk_res.1.max(atk);
        atk_res.2 =  atk_res.2.min(atk);

        time_res.0 = total_dt;
        time_res.1 = time_res.1.max(total_dt);
        time_res.2 = time_res.2.min(total_dt);

        avg_pps = if avg_pps == 0.0 {pps} else {(avg_pps + pps) / 2.0};

        println!("{BLD}Results{RST}: apm: {HLT}{}{RST}, atk: {BLD}{}{RST}, time: {BLD}{:.2}{RST}", apm, atk, total_dt);
    }
    apm_res.0  /= iter as f64;
    atk_res.0  /= iter as f64;
    time_res.0 /= iter as f32;

    let app_res = 
        (atk_res.0 as f32 / pieces as f32, atk_res.1  as f32/ pieces as f32, atk_res.2 as f32/ pieces as f32);
    println!("{BLD}=== Final Results ==={RST}:");
    println!("avg app : {HLT}{:.2}{RST}, best: {HLT}{:.2}{RST}, worst: {BLD}{:.2}{RST}", app_res.0, app_res.1, app_res.2);
    println!("avg apm : {HLT}{:.2}{RST}, best: {HLT}{:.2}{RST}, worst: {BLD}{:.2}{RST}", apm_res.0, apm_res.1, apm_res.2);
    println!("avg atk : {HLT}{:.2}{RST}, best: {HLT}{}{RST}, worst: {BLD}{}{RST}", atk_res.0, atk_res.1, atk_res.2);
    println!("avg time: {HLT}{:.2}{RST}, worst: {HLT}{:.2}{RST}, best: {BLD}{:.2}{RST}", time_res.0, time_res.1, time_res.2);
    println!("avg pps : {HLT}{:.2}{RST}", avg_pps);
}
