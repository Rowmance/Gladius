//! A chess player.

use std::fmt::{Display, Formatter, Result};

/// Represents a player.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    /// White player.
    White,

    /// Black player.
    Black,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
