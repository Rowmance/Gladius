//! A square on the chess board.

use std::fmt::{Display, Formatter};

use board::bitboard::BitBoard;
use board::file::File;
use board::rank::Rank;
use std::fmt;
use std::result::Result::Err;
use std::str::FromStr;

/// Represents a square on a board.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Square(u8);

impl Square {
    /// Creates a new instance with the given value.
    ///
    /// Will panic if debug mode and the value is not <64.
    pub fn new(val: u8) -> Self {
        debug_assert!(val < 64, "Attempt to instantiate Square with value {}", val);
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

    /// Returns the diagonal (A1-H8 direction) which passes through the square.
    pub fn diagonal(&self) -> BitBoard {
        // Shift the main diagonal as appropriate and use a mask to remove any overflowing squares.
        let main_diff = self.file().to_index() as isize - self.rank().to_index() as isize;
        let main_mask = !BitBoard::new((2 as u64).pow((8 * main_diff.abs()) as u32) - 1);
        if main_diff > 0 {
            BitBoard::new(0x8040201008040201 << main_diff) & main_mask.flip()
        } else {
            BitBoard::new(0x8040201008040201 >> main_diff.abs()) & main_mask
        }
    }

    /// Returns the antidiagonal (A8-H1 direction) which passes through the square.
    pub fn antidiagonal(&self) -> BitBoard {
        let anti_diff = (-7 + self.rank().to_index() as isize) + self.file().to_index() as isize;
        if anti_diff >= 0 {
            let anti_mask = !((2 as u64).pow(8 * (anti_diff.abs()) as u32) - 1);
            BitBoard::new((0x0102040810204080 & anti_mask) << anti_diff)
        } else {
            let anti_mask =
                0xFFFFFFFFFFFFFFFF ^ ((2 as u64).pow(8 * (8 - anti_diff.abs()) as u32) - 1);
            BitBoard::new((0x0102040810204080 & !anti_mask) >> anti_diff.abs())
        }
    }
}

// ---------------------------------------------------------------------
impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

// ---------------------------------------------------------------------
impl FromStr for Square {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut chars = s.chars();
        let file_char = chars.next().ok_or(format!("{} is not a valid square", s))?;
        let rank_char = chars.next().ok_or(format!("{} is not a valid square", s))?;
        let file = File::from_str(file_char.to_string().as_ref())?;
        let rank = Rank::from_str(rank_char.to_string().as_ref())?;
        if let Some(_) = chars.next() {
            return Err(format!("{} is not a valid square", s));
        }
        Ok(Square::from_coordinates(file, rank))
    }
}
