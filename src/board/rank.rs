//! A rank on the chess board.

use std::slice::Iter;
use std::fmt::{Formatter, Result, Display};

use board::bitboard::BitBoard;

/// Represents a rank on a chessboard.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rank {
    One, Two, Three, Four, Five, Six, Seven, Eight
}

impl Rank {
    /// Obtains a rank from the given index.
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            _ => panic!("Cannot create rank from index {}", index)
        }
    }

    /// Converts the rank to an index.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Converts the rank to a [BitBoard]
    pub fn to_bitboard(&self) -> BitBoard {
        let mut val: u64 = 0;
        val = val | 1 << (*self as u8);
        val |= val << 8;
        val |= val << 16;
        val |= val << 32;
        BitBoard(val)
    }

    /// Returns an iterator over all the ranks.
    pub fn iter() -> Iter<'static, Rank> {
        use self::Rank::*;
        static RANKS: [Rank; 8] = [One, Two, Three, Four, Five, Six, Seven, Eight];
        RANKS.into_iter()
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Rank {:?}", self)
    }
}
