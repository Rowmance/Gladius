//! A rank on the chess board.

use std::fmt::{Display, Formatter};
use std::slice::Iter;

use board::bitboard::BitBoard;
use std::fmt;
use std::str::FromStr;

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
    Eight,
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
            _ => panic!("Cannot create rank from index {}", index),
        }
    }

    /// Converts the rank to an index.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Converts the rank to a [BitBoard]
    pub fn to_bitboard(&self) -> BitBoard {
        BitBoard(0xFF << (*self as u8) * 8)
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
            Eight => None,
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
            Eight => Some(Seven),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_index() + 1)
    }
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s.to_uppercase().as_ref() {
            "1" => Ok(Rank::One),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            _ => Err(format!("Cannot convert {} to rank", s)),
        }
    }
}
