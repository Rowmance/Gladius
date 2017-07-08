//! A chess piece type.

use std::slice::Iter;
use std::fmt::{Formatter, Result, Display};

/// Represents a chess piece type.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Piece {
    /// Pawn.
    Pawn,

    /// Rook.
    Rook,

    /// Knight.
    Knight,

    /// Bishop.
    Bishop,

    /// Queen.
    Queen,

    /// King.
    King
}

impl Piece {
    /// Returns an iterator over all the pieces.
    pub fn iter() -> Iter<'static, Piece> {
        use self::Piece::*;
        static PIECES: [Piece; 6] = [Pawn, Rook, Knight, Bishop, Queen, King];
        PIECES.into_iter()
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
