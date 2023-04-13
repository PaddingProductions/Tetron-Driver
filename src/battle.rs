use std::sync::mpsc;
use std::thread;
use std::io::{Read, stdout};
use crossterm::{terminal, execute, cursor};

use tetron::*;
use crate::colors::*;
use crate::board::Board;

mod render;
mod user;

pub fn battle () {
    // Two channels, bot -> player and player -> bot.
    let (tx1, rx1) = mpsc::channel::<Option<(State, Board)>>();
    let (tx2, rx2) = mpsc::channel::<Option<(State, Board)>>();
    let mut p1 = (State::new(), Board::new(None));
    let mut p2 = (State::new(), Board::new(None));
    let mut bag = vec![];

    while p1.0.pieces.len() < 6 && p2.0.pieces.len() < 6 {
        let piece = crate::sim::draw(&mut bag);
        p1.0.pieces.push_back(piece);
        p2.0.pieces.push_back(piece);
    }

    // Runs the bot with the given state until a 'None' is received.
    // Sends the State & Move.
    let handle = thread::spawn(move || {
        loop {
            if let Some((state, mut board)) = rx1.recv().unwrap() {
                if let Some(out) = tetron::solve(&state, &config::Config::new(3, EvaluatorMode::Norm)) {
                    board.apply_move(
                        &out.1, 
                        &state.pieces[0], 
                        if state.hold == Piece::None {&state.pieces[1]} else {&state.hold}
                    );
                    tx2.send(Some((out.0, board)))
                        .expect("TX error");
                } else {
                    tx2.send(None)
                        .expect("TX error");
                }
            } else {
                break;
            }
        }
    }); 

    loop {
        let mut stdout = stdout();
        execute!(stdout, cursor::MoveTo(0,0)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        render::render(&p1.0, &p1.1, Some((1, 1)), &mut stdout);
        render::render(&p2.0, &p2.1, Some((35, 1)), &mut stdout);
        tx1.send(Some(p2.clone())).expect("TX error");

        // Get Player Move
        p1 = user::get_move(p1, &p2);

        // Extract Bot Move
        let bot_res = rx2.recv().unwrap();
        if bot_res.is_none() {
            println!("{BLD}Bot could not find path, Player Wins.{RST}");
            break;
        }
        p2 = bot_res.unwrap();

        // Process
        assign_atk(&mut p1, &mut p2);
        if check_over(&p1) {
            println!("{BLD}=== BOT WINS! ==={RST}");
            break;
        }
        if check_over(&p2) {
            println!("{BLD}=== PLAYER WINS! ==={RST}");
            break;
        }
        // Draw
        while p1.0.pieces.len() < 6 && p2.0.pieces.len() < 6 {
            let piece = crate::sim::draw(&mut bag);
            p1.0.pieces.push_back(piece);
            p2.0.pieces.push_back(piece);
        }
        println!("{}\r", p1.0);
        println!("{}\r", p1.0);
    }
    // Closing
    tx1.send(None).expect("TX error");
    println!("Awaiting Join...");
    handle.join().unwrap();
    println!("Finished.");
}


static mut P1_INC: usize = 0;
static mut P2_INC: usize = 0;

fn assign_atk (p1: &mut (State, Board), p2: &mut (State, Board)) {
    // Cancelation 
    let d = p1.0.props.atk.min(p2.0.props.atk);
    p1.0.props.atk -= d;
    p2.0.props.atk -= d;

    // Spawn garbage.
    unsafe {
        P2_INC += p1.0.props.atk as usize;
        if P2_INC > 0 {
            crate::sim::gen_garbage(&mut p2.0.field, &mut p2.1, P2_INC);
            P2_INC -= P2_INC.min(10);
        } 

        P1_INC += p2.0.props.atk as usize;
        if P1_INC > 0 {
            crate::sim::gen_garbage(&mut p1.0.field, &mut p1.1, P1_INC);
            P1_INC -= P1_INC.min(10);
        } 
    }
}

fn check_over (p: &(State, Board)) -> bool {
    if p.0.pieces.is_empty() {
        panic!("Checking state with empty piece queue.");
    }
    p.0.field.check_conflict(&Move::new(), &p.0.pieces[0])
}
