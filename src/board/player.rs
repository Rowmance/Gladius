use std::fmt::{Formatter, Result, Display};

/// Represents a player.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
    WHITE, BLACK
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Player {:?}",  self)
    }
}
