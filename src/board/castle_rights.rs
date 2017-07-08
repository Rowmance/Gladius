//! The castle rights.

use std::fmt::{Formatter, Result, Display};

/// The castle rights.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CastleRights {
    None,
    KingSide,
    QueenSide,
    Both
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Castle Rights: {:?}",  self)
    }
}
