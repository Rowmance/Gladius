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

impl Player {
    /// Returns the other player.
    pub fn other(&self) -> Self {
        match *self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
