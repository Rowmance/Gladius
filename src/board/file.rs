//! A file on the chess board.

use std::fmt::{Formatter, Result, Display};
use std::slice::Iter;

use board::bitboard::BitBoard;

/// Represents a file on a chessboard.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum File {
    A, B, C, D, E, F, G, H
}

impl File {
    /// Obtains a file from the given index.
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Cannot create file from index {}", index)
        }
    }

    /// Converts the file to an index.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Converts the file to a [BitBoard]
    pub fn to_bitboard(&self) -> BitBoard {
        BitBoard(0xFF << (*self as u8) * 8)
    }

    /// Returns an iterator over all the files.
    pub fn iter() -> Iter<'static, File> {
        use self::File::*;
        static FILES: [File; 8] = [A, B, C, D, E, F, G, H];
        FILES.into_iter()
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
