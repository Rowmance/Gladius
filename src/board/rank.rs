use std::fmt::{Formatter, Result, Display};

use board::bitboard::BitBoard;

/// Represents a rank on a chessboard.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Rank {
    ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT
}

impl Rank {
    /// Obtains a rank from the given index.
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Rank::ONE,
            1 => Rank::TWO,
            2 => Rank::THREE,
            3 => Rank::FOUR,
            4 => Rank::FIVE,
            5 => Rank::SIX,
            6 => Rank::SEVEN,
            7 => Rank::EIGHT,
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
}

impl Display for Rank {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Rank {:?}",  self)
    }
}
