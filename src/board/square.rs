//! A square on the chess board.

use std::fmt::{Formatter, Result, Display};

use board::bitboard::BitBoard;
use board::file::File;
use board::rank::Rank;

/// Represents a square on a board.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Square(u8);

impl Square {
    /// Creates a new instance with the given value.
    ///
    /// Will panic if debug mode and the value is not <64.
    pub fn new(val: u8) -> Self {
        debug_assert!(val < 64, "Attempt to initiate Square with value {}", val);
        Square(val)
    }

    /// Returns the file of the square.
    pub fn file(&self) -> File {
        File::from_index(self.0 % 8)
    }

    /// Returns the rank of the square.
    pub fn rank(&self) -> Rank {
        Rank::from_index(self.0 / 8)
    }

    /// Returns the square from the opposite players point of view.
    pub fn flip(&self) -> Self {
        Square::new(63 - self.0)
    }

    /// Returns the square mirrored horizontally
    pub fn mirror_horizontal(&self) -> Self {
        Square::new(8 * (7 - (self.0 / 8)) + (self.0 % 8))
    }
    
    /// Returns the square mirrored accross the A1-H8 diagonal
    pub fn mirror_diag(&self) -> Self {
        Square::new(8 * (self.0 % 8) + (self.0 / 8))
    }

    /// Returns a bitboard with only the square marked.
    pub fn to_bitboard(&self) -> BitBoard {
        BitBoard::new(1 << self.0)
    }

    /// Returns the index of the square, compatible with [BitBoard] representation.
    pub fn to_index(&self) -> u8 {
        self.0
    }

    /// Creates a new intance from the given file and rank.
    pub fn from_coordinates(file: File, rank: Rank) -> Self {
        Square(file.to_index() + rank.to_index() * 8)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
