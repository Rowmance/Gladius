use std::collections::HashMap;

use board::piece::Piece;
use board::bitboard::BitBoard;
use board::player::Player;
use board::square::Square;

/// Represents a complete state of a chess board.
/// * whether en-passant is possible, and where
pub struct Board {
    /// A map of positions of each of the white player pieces.
    white_pieces: HashMap<Piece, BitBoard>,

    /// A map of positions of each of the black player pieces.
    black_pieces: HashMap<Piece, BitBoard>,

    /// The player whose turn it is.
    player_turn: Player,

    /// The square an en-passant capture is available on, if any.
    en_passant: Option<Square>,

    /// The number of half-turns since the last capture or pawn advance.
    draw_half_turns: usize,

    /// The number of full turns elapsed.
    full_turns: usize
}
