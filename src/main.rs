mod board;
mod sim;

use tetron::*;

use std::io;

pub use board::Board;

pub mod colors {
    pub const RST: &str = "\x1b[0m";
    pub const BLD: &str = "\x1b[1m";
    pub const HLT: &str = "\x1b[48;5;226m";
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
    pub(crate) use color;
    pub(crate) use piece_color;
}
use crate::colors::*;

fn main() {
    println!("{HLT}==={{ Tetron CLI }}==={RST}");
    println!("\n{BLD}Commands:{RST}");
    println!("- sandbox <mode: [{BLD}norm{RST}, atk, ds]> <(num)lines =40>");
    println!("- cheese_exam <(num)iters {BLD}40{RST}> <(num)lines =40> <opts: [log]>");
    println!("- attack_exam <(num)iters {BLD}5{RST}> <(num)pieces =100> <opts: [log]>");

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Error on STDIN.");
        buf.pop();

        let args: Vec<&str> = buf.split(" ").collect();
        
        println!("command: {buf}. args: {:?}", args);
        match args[0] {
            "sandbox" => {
                let mode = if args.len() < 2 {EvaluatorMode::Norm} else {match args[1] {
                    "atk" => EvaluatorMode::Attack,
                    "ds" => EvaluatorMode::DS,
                    "norm" => EvaluatorMode::Norm,
                    _ => EvaluatorMode::Norm,
                }};
                let iters = if args.len() < 3 {100} else {args[2].parse().unwrap()};
                sim::sandbox_run(iters, Some(mode));
            },
            "cheese_exam" => {
                let iter = if args.len() > 1 {args[1].parse().unwrap()} else {10};
                let lines = if args.len() > 2 {args[2].parse().unwrap()} else {18};
                let log = if args.len() > 3 {args[3] == "log"} else {false};
                sim::cheese_exam(iter, lines, log);
            }
            "attack_exam" => {
                let iter = if args.len() > 1 {args[1].parse().unwrap()} else {5};
                let lines = if args.len() > 2 {args[2].parse().unwrap()} else {40};
                let log = if args.len() > 3 {args[3] == "log"} else {false};
                sim::attack_exam(iter, lines, log);
            }
            _ => continue
        }
        break;
    }
}