//! Generation of semi-legal moves.
//! These are moves which, unlike those in [basic_moves], consider the positions
//! of other pieces. These move generation functions do not consider check or checkmate.

use board::square::Square;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::file::File;
use board::bitboard::BitBoard;
use game::basic_moves;
use std::num::Wrapping;

/// Returns the moves a given pawn can make.
pub fn pawn_moves(square: Square, player: Player, blockers: BitBoard) -> BitBoard {
    let double_blockers = match player {
        Player::White => blockers | blockers << 8,
        Player::Black => blockers | blockers >> 8
    };
    basic_moves::pawn_moves(square, player) & !double_blockers
}

/// Returns the attacks a given pawn can make.
pub fn pawn_attacks(square: Square, player: Player, opponent_pieces: BitBoard) -> BitBoard {
    basic_moves::pawn_attacks(square, player) & opponent_pieces
}

// -----------------------------------
/// Returns the moves a given knight can make.
pub fn knight_moves(square: Square, blockers: BitBoard) -> BitBoard {
    basic_moves::knight(square) & !blockers
}

/// Returns the attacks a given knight can make.
pub fn knight_attacks(square: Square, opponent_pieces: BitBoard) -> BitBoard {
    basic_moves::knight(square) & opponent_pieces
}

// -----------------------------------
/// Returns the moves a given king can make.
pub fn king_moves(square: Square, blockers: BitBoard) -> BitBoard {
    basic_moves::king(square) & !blockers
}

/// Returns the attacks a given king can make.
pub fn king_attacks(square: Square, opponent_pieces: BitBoard) -> BitBoard {
    basic_moves::king(square) & opponent_pieces
}

// -----------------------------------
/// Returns the combination of moves and captures a rook can make, assuming the blockers 
/// can all be captured.
fn rook_all_moves(square: Square, blockers: BitBoard) -> BitBoard {
    // gets the moves in the upward direction only
    fn rook_forward_moves(square: Square, blockers: BitBoard) -> BitBoard {
        let file_mask = square.file().to_bitboard();
        let pot_blockers = blockers & file_mask;

        let difference = pot_blockers - BitBoard::new((Wrapping(square.to_bitboard().to_u64()) * Wrapping(2)).0);
        let changed = difference ^ blockers;
        changed & file_mask
    }
    
    let up = rook_forward_moves(
        square,
        blockers);
    let down = rook_forward_moves(
        square.mirror_horizontal(),
        blockers.mirror_horizontal()).mirror_horizontal();
    let left = rook_forward_moves(
        square.mirror_diag(),
        blockers.mirror_diag()).mirror_diag();
    let right = rook_forward_moves(
        square.mirror_diag().mirror_horizontal(),
        blockers.mirror_diag().mirror_horizontal()).mirror_horizontal().mirror_diag();
    
    up | down | left | right
}

/// Returns the moves a given rook can make
pub fn rook_moves(square: Square, blockers: BitBoard) -> BitBoard {
    rook_all_moves(square, blockers) & !blockers
}

/// Returns the attacks a given rook can make
pub fn rook_attacks(square: Square, own_pieces: BitBoard, opponent_pieces: BitBoard) -> BitBoard {
    rook_all_moves(square, own_pieces | opponent_pieces) & opponent_pieces
}

// -----------------------------------
