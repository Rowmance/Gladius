//! A chess player.

use std::fmt::{Formatter, Result, Display};

/// Represents a player.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    White, Black
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Player {:?}",  self)
    }
}
