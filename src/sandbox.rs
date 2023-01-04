use tetron::*;
use rand::Rng;
use std::time::{Instant};
use std::io;
use crate::board::*;

const RST: &str = "\x1b[0m";
const BLD: &str = "\x1b[1m";
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

pub fn run () {
    let mut board = Board::new();
    let mut state = State::new();
    let mut bag = vec![Piece::J, Piece::L, Piece::S, Piece::Z, Piece::T, Piece::I, Piece::O];
    
    fn draw (bag: &mut Vec<Piece>) -> Piece {
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
        let out: (State, Move, f32) = solve(&state, 1);
        let dt = start.elapsed().as_micros();
        avg_dt = if avg_dt == 0 {dt} else {(avg_dt + dt) / 2};

        println!("Time consumed: {}{}{}us", BLD, dt, RST);
        println!("Avg benchmark: {}{}{}us", BLD, avg_dt, RST);

        // Apply move to colored board
        board.apply_move(&out.1, &state.pieces[0], if state.hold == Piece::None {&state.pieces[1]} else {&state.hold});

        // Log out result
        println!("result score: {BLD}{}{RST}", out.2);
        render(&board, &out.0);
        state = out.0;

        // Read user input
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "q" => break,
            _ => continue,
        }
    }
}

fn render (board: &Board, state: &State) {
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
