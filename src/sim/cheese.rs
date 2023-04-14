use tetron::{solve, Piece, State, EvaluatorMode, config};

use std::time::Instant;
use std::io::{self, Write};

use crate::board::Board;
use crate::colors::*;


fn run (lines: usize, log: bool) -> (u32, f32, u128) {
    let mut state = State::new();
    let mut board = Board::new(Some(&state.field));

    // Gen cheese 
    let mut cheese_row = 10.min(lines);
    let mut cheese_clears = 0;
    let mut piece_cnt: u128 = 0;
    for _ in (20 - lines.min(10))..20 {
        super::gen_garbage(&mut state.field, &mut board, 1);
    }

    let mut bag = vec![];
    
    let mut avg_dt: u128 = 0;
    let mut total_dt: f32 = 0.0;
    while state.pieces.len() < 6 {
        state.pieces.push_back(super::draw(&mut bag));
    }
    println!("{}", board);
    while cheese_clears < lines {
        // Draw pieces
        while state.pieces.len() < 6 {
            state.pieces.push_back(super::draw(&mut bag));
        }
        
        // Solve & Bench
        let start = Instant::now();        
        if let Some(out) = solve(&state, &config::Config::new(3, EvaluatorMode::DS)) {
            let dt = start.elapsed().as_micros();
            total_dt += dt as f32 / 1_000_000.0;
            avg_dt = (avg_dt * piece_cnt + dt) / (piece_cnt + 1);

            // Apply move to colored board
            board.apply_move(&out.1, &state.pieces[0], if state.hold == Piece::None {&state.pieces[1]} else {&state.hold});

            // Log out result
            if log {
                println!("Time consumed: {}{}{}us", BLD, dt, RST);
                println!("Avg benchmark: {}{}{}us", BLD, avg_dt, RST);
                super::render(&board, &out.0);
            }
            // Process cheese
            for y in (20-cheese_row)..20 {
                if out.0.props.clears & 1 << y > 0 {
                    cheese_row -= 1;
                    cheese_clears += 1;
                }
            }
            state.field = out.0.field;
            state.pieces = out.0.pieces;
            state.hold = out.0.hold;

            // Spawn Chese 
            if out.0.props.clears == 0 && cheese_row < 10 && cheese_clears + cheese_row < lines {
                while cheese_row < 10 {
                    super::gen_garbage(&mut state.field, &mut board, 1);
                    cheese_row += 1;
                }
            }
        } else {
            println!("{BLD}No results found, game over.{RST}");
            break;
        }
        print!(".");
        io::stdout().flush().unwrap();
        piece_cnt += 1;
    }
    println!();
    (piece_cnt as u32, total_dt, avg_dt)
}

pub fn cheese_exam (iter: usize, lines: usize, log: bool) {
    println!("{HLT}--CHEESE EXAM--{RST}");
    println!("{BLD}--iters: {iter}, lines: {lines}--{RST}");

    // Results: (average, minimum, maximum)
    let mut pieces_res: (f64, u32, u32) = (0.0, 0, u32::MAX);
    let mut time_res: (f32, f32, f32) = (0.0, 0.0, 0.0);
    let mut avg_pps: f64 = 0.0;

    // Calc & Store Results after each iteration
    for _ in 0..iter {
        let (pieces, total_dt, avg_dt) = run(lines, log);
        let pps: f64 = 1.0 / (avg_dt as f64 / 1_000_000.0);

        pieces_res.0 += pieces as f64;
        pieces_res.1 = pieces_res.1.max(pieces);
        pieces_res.2 = pieces_res.2.min(pieces);

        time_res.0 += total_dt;
        time_res.1 = time_res.1.max(total_dt);
        time_res.2 = time_res.2.min(total_dt);

        avg_pps = if avg_pps == 0.0 {pps} else {(avg_pps + pps) / 2.0};

        println!("{BLD}Results{RST}: pieces: {HLT}{}{RST}, time: {BLD}{}{RST}, pps: {BLD}{:.2}{RST}", pieces, total_dt, pps);
    }
    pieces_res.0 /= iter as f64;
    time_res.0   /= iter as f32;

    println!("{BLD}Final Results{RST}:");
    println!("avg pieces: {HLT}{}{RST}, worst: {HLT}{}{RST}, best: {BLD}{}{RST}", pieces_res.0, pieces_res.1, pieces_res.2);
    println!("avg time: {HLT}{}{RST}, worst: {HLT}{}{RST}, best: {BLD}{}{RST}", time_res.0, time_res.1, time_res.2);
    println!("avg pps: {HLT}{:.2}{RST}", avg_pps);
}
