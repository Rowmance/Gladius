//! A rank on the chess board.

use std::slice::Iter;
use std::fmt::{Formatter, Result, Display};

use board::bitboard::BitBoard;

/// Represents a rank on a chessboard.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Rank {
    /// First rank.
    One,

    /// Second rank.
    Two,

    /// Third rank.
    Three,

    /// Fourth rank.
    Four,

    /// Fifth rank.
    Five,

    /// Sixth rank.
    Six,

    /// Seventh rank.
    Seven,

    /// Eighth rank.
    Eight
}

impl Rank {
    /// Obtains a rank from the given index.
    pub fn from_index(index: u8) -> Self {
        use self::Rank::*;
        match index {
            0 => One,
            1 => Two,
            2 => Three,
            3 => Four,
            4 => Five,
            5 => Six,
            6 => Seven,
            7 => Eight,
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

    /// Returns the next rank, or none if on the last rank.
    pub fn next(&self) -> Option<Rank> {
        use self::Rank::*;
        match *self {
            One => Some(Two),
            Two => Some(Three),
            Three => Some(Four),
            Four => Some(Five),
            Five => Some(Six),
            Six => Some(Seven),
            Seven => Some(Eight),
            Eight => None
        }
    }

    /// Returns the previous rank, or none if on the first rank.
    pub fn prev(&self) -> Option<Rank> {
        use self::Rank::*;
        match *self {
            One => None,
            Two => Some(One),
            Three => Some(Two),
            Four => Some(Three),
            Five => Some(Four),
            Six => Some(Five),
            Seven => Some(Six),
            Eight => Some(Seven)
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_index() + 1)
    }
}
