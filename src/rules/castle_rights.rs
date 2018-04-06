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
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Castle Rights: {:?}", self)
    }
}
