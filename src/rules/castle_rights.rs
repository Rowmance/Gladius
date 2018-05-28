//! The castle rights.

use std::fmt::{Display, Formatter, Result};

/// The castle rights.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CastleRights {
    /// No castle available.
    None,

    /// King-side castle available.
    KingSide,

    /// Queen-side castle available.
    QueenSide,

    /// King-side and queen-side castle available.
    Both,
}

impl CastleRights {
    /// Returns true if a queen side castle is available.
    pub fn is_queen_side_available(&self) -> bool {
        *self == CastleRights::Both || *self == CastleRights::QueenSide
    }

    /// Returns true if a king side castle is available.
    pub fn is_king_side_available(&self) -> bool {
        *self == CastleRights::Both || *self == CastleRights::KingSide
    }

    /// Returns castle rights without the king side.
    pub fn without_king_side(&self) -> CastleRights {
        match *self {
            CastleRights::Both => CastleRights::QueenSide,
            CastleRights::KingSide => CastleRights::None,
            _ => panic!(format!("Cannot remove king side castle rights from {}", self)),
        }
    }

    /// Returns castle rights without the queen side.
    pub fn without_queen_side(&self) -> CastleRights {
        match *self {
            CastleRights::Both => CastleRights::KingSide,
            CastleRights::QueenSide => CastleRights::None,
            _ => panic!(format!("Cannot remove queen side castle rights from {}", self)),
        }
    }
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Castle Rights: {:?}", self)
    }
}
