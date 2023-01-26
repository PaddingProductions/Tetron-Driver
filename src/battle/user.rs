use std::io::{self, stdout};
use std::time::Instant;
use crossterm::{cursor, terminal, execute};

use tetron::*;
use crate::board::Board;
use super::render::render;

use super::*;

// Takes in the State and boards before the move.
// Returns the moved State and the respective Move.
pub fn get_move (p1: (State, Board), p2: &(State, Board)) -> (State, Board) {
    let mut state = p1.0;
    let mut board = p1.1;

    if state.pieces.is_empty() {
        panic!("No pieces in state.");
    }
    let mut mov = Move::new();
    let piece = state.pieces[0];
    let hold = if state.hold != Piece::None {&state.hold} else {&state.pieces[1]};

    // DAS
    let mut das_d = Key::Left;
    let mut das_t: Option<Instant> = None;

    terminal::enable_raw_mode().expect("Err on raw mode activation");
    let mut stdout = stdout();
    loop {
        if let Some(key) = get_input() {
            // If held, reset Move
            if key == Key::Hold {
                mov = Move::new();
                mov.hold = true;
            }
            // DAS. Check if elapsed time > DAS. 
            if let Some(t) = das_t { 
                // If direction is the same and is 20 ms in range of "DAS"
                if key == das_d && t.elapsed().as_millis().abs_diff(250) < 20 {
                    mov.apply_key(if das_d == Key::Left {&Key::DASLeft} else {&Key::DASRight}, &state.field, &piece, hold);
                }
                das_t = None;
            }
            if key == Key::Left || key == Key::Right {
                das_d = key.clone();
                das_t = Some(Instant::now());
            } 
            mov.apply_key(&key, &state.field, &piece, &hold);
            
            execute!(stdout, cursor::MoveTo(0,0)).unwrap();
            // Render move preview
            {
                let mut board: Board = board.clone();
                let mut previews = state.pieces.clone();
                
                board.apply_move(&mov, &state.pieces[0], hold);
        
                // Identify preview pieces.
                previews.pop_front();
                if mov.hold && state.hold == Piece::None {
                    previews.pop_front();
                }
                render(&state, &board, Some((1, 1)), &mut stdout);
            }
            render(&p2.0, &p2.1, Some((35, 1)), &mut stdout);

            if key == Key::HardDrop {
                break;
            }
        } else {
            break;
        }
    }
    board.apply_move(&mov, &state.pieces[0], if state.hold == Piece::None {&state.pieces[1]} else {&state.hold});
    state = state.clone_as_child(
        state.field.apply_move(&mov, &piece, &hold).unwrap(),
        &mov
    );

    // end raw mode
    terminal::disable_raw_mode().expect("Err on raw mode deactivation");
    (state, board)
}


fn get_input () -> Option<Key> {
    // Get first byte
    let mut buf: [u8; 4] = [0; 4];
    let len = io::stdin().read(&mut buf).expect("Error on stdin");

    if len == 0 {
        return None
    }
    // Check Escape sequence 
    if buf[0] == 27 {
        print!("{}\n\r", buf[len-1]);
        return match buf[len-1] {
            68 => Some(Key::Left),
            67 => Some(Key::Right),
            65 => Some(Key::Cw),
            66 => Some(Key::SoftDrop),
            _ => None,
        }
    }
    match buf[0] {
        32 => Some(Key::HardDrop),
        99 => Some(Key::Hold),
        122 => Some(Key::Ccw),
        _ => None,
    }
}