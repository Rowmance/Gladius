//! A movement of a piece

use board::square::Square;
use board::piece::Piece;

/// A movement of a piece (a 'move')
pub struct Move {
    /// The square the piece moved from.
    pub origin: Square,

    /// The square the piece moved to.
    pub target: Square,

    /// The type of the piece which moved.
    pub piece: Piece,

    /// True if the move was a capture.
    pub capture: bool,

    /// True if the capture was en-passant.
    pub en_passant: bool,

    /// Present and containing the target piece if the move was a promotion.
    pub promotion: Option<Piece>,

    /// True if the move was a castle
    pub castle: bool,
}
