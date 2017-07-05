use std::collections::HashMap;
use std::fmt::{Formatter, Result, Display};

use board::piece::Piece;
use board::bitboard::BitBoard;
use board::player::Player;
use board::square::Square;

/// Represents a complete state of a chess board.
/// * whether en-passant is possible, and where
pub struct Board {
    /// A map of positions of each of the white player pieces.
    pub white_pieces: HashMap<Piece, BitBoard>,

    /// A map of positions of each of the black player pieces.
    pub black_pieces: HashMap<Piece, BitBoard>,

    /// The player whose turn it is.
    pub player_turn: Player,

    /// The square an en-passant capture is available on, if any.
    pub en_passant: Option<Square>,

    /// The number of half-turns since the last capture or pawn advance.
    pub draw_half_turns: usize,

    /// The number of full turns elapsed.
    pub full_turns: usize
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        unimplemented!()
    }
}
