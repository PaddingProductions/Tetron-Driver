use tetron::{Field, Piece, State};
use crate::board::Board;
use crate::colors::*;

use rand::Rng;

mod cheese;
mod attack;
mod sandbox;
mod backfire;

pub use sandbox::sandbox_run;
pub use cheese::cheese_exam;
pub use attack::attack_exam;
pub use backfire::backfire_exam;


pub fn draw (bag: &mut Vec<Piece>) -> Piece {
    if bag.is_empty() {
        bag.push(Piece::J);
        bag.push(Piece::L);
        bag.push(Piece::S);
        bag.push(Piece::Z);
        bag.push(Piece::T);
        bag.push(Piece::I);
        bag.push(Piece::O);
    }
    let i = rand::thread_rng().gen_range(0..bag.len());
    let p = bag.remove(i);
    p
}

pub fn gen_garbage (field: &mut Field, board: &mut Board, lines: usize) {
    let lines = lines.min(10);
    static mut PREV: u8 = 0;

    let mut rng = rand::thread_rng();
    let i: u8 = unsafe {
        let mut i: u8 = PREV;
        while i == PREV { i = rng.gen_range(0..10); }
        PREV = i;
        i
    };
    let nrow: u16 = ((1 << 10) - 1) - (1 << i);

    for y in lines..20 {
        field.m[y-lines] = field.m[y];
        board.m[y-lines] = board.m[y];
    }
    for y in (20-lines)..20 {
        field.m[y] = nrow;
        for x in 0..10 {
            board.m[y][x] = if nrow & 1 << x > 0 {Piece::L} else {Piece::None};
        }
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