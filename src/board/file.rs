//! A file on the chess board.

use std::fmt::{Formatter, Result, Display};
use std::slice::Iter;

use board::bitboard::BitBoard;

/// Represents a file on a chessboard.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum File {
    /// A file.
    A,

    /// B file.
    B,

    /// C file.
    C,

    /// D file.
    D,

    /// E file.
    E,

    /// F file.
    F,

    /// G file.
    G,

    /// H file.
    H
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
        let mut val: u64 = 0;
        val = val | 1 << (*self as u8);
        val |= val << 8;
        val |= val << 16;
        val |= val << 32;
        BitBoard(val)
    }

    /// Returns an iterator over all the files.
    pub fn iter() -> Iter<'static, File> {
        use self::File::*;
        static FILES: [File; 8] = [A, B, C, D, E, F, G, H];
        FILES.into_iter()
    }

    /// Returns the next file, or none if on the last file.
    pub fn next(&self) -> Option<File> {
        use self::File::*;
        match *self {
            A => Some(B),
            B => Some(C),
            C => Some(D),
            D => Some(E),
            E => Some(F),
            F => Some(G),
            G => Some(H),
            H => None
        }
    }

    /// Returns the previous file, or none if on the first file.
    pub fn prev(&self) -> Option<File> {
        use self::File::*;
        match *self {
            A => None,
            B => Some(A),
            C => Some(B),
            D => Some(C),
            E => Some(D),
            F => Some(E),
            G => Some(F),
            H => Some(G)
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
