mod board;
mod sim;
mod battle;

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
                Piece::J => "\x1b[48;5;20m", // blue
                Piece::L => "\x1b[48;5;208m", // bright red / orange
                Piece::S => "\x1b[48;5;46m", // green
                Piece::Z => "\x1b[48;5;9m", // red
                Piece::T => "\x1b[45;1m", // magenta
                Piece::I => "\x1b[48;5;51m", // cyan
                Piece::O => "\x1b[48;5;226m", // yellow
            }
        };
    }
    pub(crate) use piece_color;
}
use crate::colors::*;


fn main() {
    println!("{HLT}==={{ Tetron CLI }}==={RST}");
    println!("\n{BLD}Commands:{RST}");
    println!("=> sandbox <mode: [{BLD}norm{RST}, atk, ds]> <(num)pieces ={BLD}100{RST}>");
    println!("=> cheese_exam <(num)iters ={BLD}40{RST}> <(num)lines ={BLD}40{RST}> <opts: [log]>");
    println!("=> attack_exam <(num)iters ={BLD}5{RST}> <(num)pieces ={BLD}100{RST}> <opts: [log]>");
    println!("=> backfire_exam <(num)iters ={BLD}2{RST}> <(num)pieces ={BLD}100{RST}> <opts: [{BLD}log{RST}]>");
    println!("=> vs ## {BLD} LEGACY {RST} ##");

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Error on STDIN.");
        buf.pop();

        
        let (flags, args) = {
            let v: Vec<&str> = buf.split(" ").collect();
            let mut flags = vec![];
            let mut args = vec![];
            for s in v {
                if s.starts_with('-') { flags.push(s); }
                else { args.push(s); }
            }
            (flags, args)
        };
        
        println!("command: {buf}. args: {:?}", args);
        match args[0] {
            "vs" => {
                battle::battle();
            }
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
                let log = flags.contains(&"-l");
                sim::cheese_exam(iter, lines, log);
            }
            "attack_exam" => {
                let iter = if args.len() > 1 {args[1].parse().unwrap()} else {5};
                let lines = if args.len() > 2 {args[2].parse().unwrap()} else {100};
                let log = flags.contains(&"-l");
                sim::attack_exam(iter, lines, log);
            }
            "backfire_exam" => {
                let iter = if args.len() > 1 {args[1].parse().unwrap()} else {2};
                let lines = if args.len() > 2 {args[2].parse().unwrap()} else {100};
                let log = flags.contains(&"-l");
                sim::backfire_exam(iter, lines, log);
            }
            _ => continue
        }
        break;
    }
}