use std::fmt::{Formatter, Result, Display};

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
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}",  self)
    }
}
