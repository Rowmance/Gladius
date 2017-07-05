use std::fmt::{Formatter, Result, Display};

/// Represents a chess piece type.
#[derive(PartialEq, Clone, Copy, Debug, Hash)]
pub enum Piece {
    PAWN, ROOK, KNIGHT, BISHOP, QUEEN, KING
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}",  self)
    }
}
