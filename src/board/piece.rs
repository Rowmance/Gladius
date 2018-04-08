//! A chess piece type.

use std::fmt::{Display, Formatter, Result};
use std::vec::IntoIter;

/// Represents a chess piece type.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    King,
}

impl Piece {
    /// Returns an iterator over all the pieces.
    pub fn iter() -> IntoIter<Piece> {
        use self::Piece::*;
        vec![Pawn, Rook, Knight, Bishop, Queen, King].into_iter()
    }

    /// Returns an iterator over all the pieces except the pawn (including the king)
    pub fn iter_non_pawn() -> IntoIter<Piece> {
        use self::Piece::*;
        vec![Rook, Knight, Bishop, Queen, King].into_iter()
    }

    /// Returns an iterator over all the non-pawn and non-king pieces.
    pub fn iter_pieces() -> IntoIter<Piece> {
        use self::Piece::*;
        vec![Rook, Knight, Bishop, Queen].into_iter()
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
