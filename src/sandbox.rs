use tetron::*;
use rand::Rng;
use std::time::{Instant};
use std::{thread, time::Duration};
use std::io;
use crate::board::*;
use crate::colors::*;


pub fn draw (bag: &mut Vec<Piece>) -> Piece {
    let i = rand::thread_rng().gen_range(0..bag.len());
    let p = bag.remove(i);
    if bag.is_empty() {
        bag.push(Piece::J);
        bag.push(Piece::L);
        bag.push(Piece::S);
        bag.push(Piece::Z);
        bag.push(Piece::T);
        bag.push(Piece::I);
        bag.push(Piece::O);
    }
    p
}



pub fn sandbox_bench_fn () -> u128 {
    const ITERS: i32 = 100;

    let mut state = State::new();
    let mut bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O];
    
    let mut avg_dt = 0;
    while state.pieces.len() < 6 {
        state.pieces.push_back(draw(&mut bag));
    }
    for _ in 0..ITERS {
        // Draw pieces
        while state.pieces.len() < 6 {
            state.pieces.push_back(draw(&mut bag));
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

pub fn run (mode: Option<EvaluatorMode>) {
    let mut state = State::new();
    
    state.field.m = [   
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b0_0_0_0_0_0_0_0_0_0,
        0b1_1_1_0_1_1_1_1_1_1,
        0b1_1_1_1_1_1_0_1_1_1,
        0b1_1_1_1_1_0_1_1_1_1,
        0b1_0_1_1_1_1_1_1_1_1,
        0b1_1_1_1_1_1_1_1_1_1,
        0b0_1_1_1_1_1_1_1_1_1,
        0b0_1_1_1_1_1_1_1_1_1,
        0b0_1_1_1_1_1_1_1_1_1,
        0b0_1_1_1_1_1_1_1_1_1,
        0b1_1_1_0_1_1_1_1_1_1,
    ]; 
    let mut board = Board::new(Some(&state.field));
    let mut bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O];
    
    let mut avg_dt = 0;
    while state.pieces.len() < 6 {
        state.pieces.push_back(draw(&mut bag));
    }
    println!("init state:\n{}", state);
    loop {
        // Draw pieces
        while state.pieces.len() < 6 {
            state.pieces.push_back(draw(&mut bag));
        }

        // Solve & Bench
        let start = Instant::now();        
        if let Some(out) = solve(&state, 2, mode) {
            let dt = start.elapsed().as_micros();
            avg_dt = if avg_dt == 0 {dt} else {(avg_dt + dt) / 2};

            println!("Time consumed: {}{}{}us", BLD, dt, RST);
            println!("Avg benchmark: {}{}{}us", BLD, avg_dt, RST);

            // Apply move to colored board
            board.apply_move(&out.1, &state.pieces[0], if state.hold == Piece::None {&state.pieces[1]} else {&state.hold});

            // Log out result
            println!("result score: {BLD}{}{RST}", out.2);
            render(&board, &out.0);


            state.field = out.0.field;
            state.pieces = out.0.pieces;
            state.hold = out.0.hold;
        } else {
            println!("{BLD}No results found, game over.{RST}");
            break;
        }
        /* 
        // Read user input
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "q" => break,
            _ => continue,
        }
        */
        thread::sleep(Duration::from_millis(300));
    }
}

pub fn render (board: &Board, state: &State) {
    for y in 0..20 {
        for x in 0..10 {
            if board.m[y][x] != Piece::None {
                let c = piece_color!(board.m[y][x]);
                print!("{}  {}", c, RST);
            } else {
                print!(". ");
            }
        }
        print!(" ");
        match y {
            0 => print!("b2b:   {BLD}{:>2}{RST}", state.props.b2b),
            1 => print!("combo: {BLD}{:>2}{RST}", state.props.combo),
            3 => print!("hold:  {BLD}{:?}{RST}", state.hold),
            4 => print!("queue:"),
            5..=9 => if state.pieces.len() > y-5 {
                print!("{BLD}{:?}{RST}", state.pieces[y-5])
            },
            _ => ()
        };
        println!();
    }
    println!();
}
