use std::io::Stdout;
use crossterm::{cursor, execute};

use tetron::*;
use crate::board::Board;
use crate::colors::*;

// Current state, before the move 
// Board of current state, before the move
// Move to be previewed
pub fn render (state: &State, board: &Board, offset: Option<(u16, u16)>, stdout: &mut Stdout) {
    if let Some(offset) = offset {
        execute!(stdout, cursor::MoveTo(offset.0, offset.1)).unwrap();
    } 

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
        if let Some(offset) = offset {
            execute!(stdout, cursor::MoveTo(offset.0, offset.1 + y as u16)).unwrap();
        } else {
            print!("\r\n");
        }
    }
    print!("\r\n");
}
