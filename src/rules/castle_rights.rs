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

impl Display for CastleRights {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Castle Rights: {:?}", self)
    }
}
