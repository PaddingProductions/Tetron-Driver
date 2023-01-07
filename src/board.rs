use tetron::{Piece, Move, Field};
use tetron::field::{PIECE_MAP, reverse_bin};
use crate::colors::*;
use std::fmt;

pub struct Board {
    pub m: [[Piece; 10]; 20]
}

impl Board {
    pub fn new (field: Option<&Field>) -> Self {
        if let Some(field) = field {
            let mut m = [[Piece::None; 10]; 20];
            for y in 0..20 {
                for x in 0..10 {
                    if field.m[y] & (1 << x) > 0 {
                        m[y][x] = Piece::J;
                    }
                }
            }
            Self {
                m
            }
        } else {
            Self {
                m: [[Piece::None; 10]; 20],
            }
        }
    }

    pub fn to_field (self: &Self) -> Field {
        let mut field = Field::new();
        for y in 0..20 {
            for x in 0..10 {
                if self.m[y][x] != Piece::None {
                    field.m[y] |= 1 << x;
                }
            }
        };
        field
    }
    
    pub fn apply_move (self: &mut Self, m: &Move, piece: &Piece, hold: &Piece) {
        let p = if m.hold {hold} else {piece};
        let map: &u32 = &PIECE_MAP[*p as usize][m.r as usize];
        let n: i8 = if *p == Piece::I {5} else {3};
        let c_x: i8 = m.x - n/2;
        let c_y: i8 = m.y - n/2;
        let mask = (1 << n) - 1;
        
        for y in 0..n {
            // The bits representing a single row of the piece map
            let shift: u8 = (n * (n - 1 - y)) as u8;
            let bitseg: u16 = reverse_bin( (( map & (mask << shift) ) >> shift) as u16 , n as u8 );
            //dev_log!("c_x: {c_x}, map: {:09b}, bitseg: {:05b}", PIECE_MAP[*p as usize][m.r as usize], bitseg);

            // If empty row on piece map
            if bitseg == 0 {
                continue;
            }
            // If out of board on upper edge
            if  c_y + y < 0 {
                panic!("@ Field.apply_move: out of board on upper edge");
            }
            // If out of board on bottom edge
            if c_y + y >= 20 {
                panic!("@ Field.apply_move: out of board on bottom edge");
            }
            // If out of board on left edge
            if c_x < 0 && bitseg & (1 << (-c_x) - 1) > 0  {
                panic!("@ Field.apply_move: out of board on left edge");
            }
            // Shift according to c_x
            let bitseg = if c_x > 0 { bitseg << c_x } else { bitseg >> -c_x };
            //dev_log!("c_x: {}, final bitseg: {:05b}", c_x, bitseg);
            // If out of board on right edge
            if bitseg > (1 << 10)-1 {
                panic!("@ Field.apply_move: out of board on right edge");
            }
            for x in 0..10 {
                if (1 << x) & bitseg > 0 {
                    self.m[(c_y + y) as usize][x] = *p;
                }
            }
        };

        // Clear 
        let mut clears: usize = 0;
        for y in (0..20).rev() {
            let mut clear: bool = true;
            for x in 0..10 {
                if self.m[y][x] == Piece::None {
                    clear = false;
                }
                if clears > 0 {
                    self.m[y+clears][x] = self.m[y][x];
                    self.m[y][x] = Piece::None;
                }
            }
            if clear {
                clears += 1;
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..20 {
            for x in 0..10 {
                if self.m[y][x] != Piece::None {
                    let c = piece_color!(self.m[y][x]);
                    write!(f, "{}  {}", c, RST)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}