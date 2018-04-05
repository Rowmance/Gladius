//! Generation of semi-legal moves.
//! These are moves which, unlike those in [basic_moves], consider the positions
//! of other pieces. These move generation functions do not consider check or checkmate.

use board::square::Square;
use board::player::Player;
use board::bitboard::BitBoard;
use rules::basic_moves;
use std::num::Wrapping;
use board::piece::Piece;

impl Piece {
    /// Returns the possible moves of the piece.
    pub fn moves(&self, square: Square, player: Player, blockers: BitBoard) -> BitBoard {
        match *self {
            Piece::Pawn => pawn_moves(square, player, blockers),
            Piece::Rook => rook_moves(square, blockers),
            Piece::Knight => knight_moves(square, blockers),
            Piece::Bishop => bishop_moves(square, blockers),
            Piece::Queen => queen_moves(square, blockers),
            Piece::King => king_moves(square, blockers),
        }
    }

    /// Returns the possible attacks of the piece.
    pub fn attacks(&self, square: Square, player: Player, own_pieces: BitBoard, opponent_pieces: BitBoard) -> BitBoard {
        match *self {
            Piece::Pawn => pawn_attacks(square, player, opponent_pieces),
            Piece::Rook => rook_attacks(square, own_pieces, opponent_pieces),
            Piece::Knight => knight_attacks(square, opponent_pieces),
            Piece::Bishop => bishop_attacks(square, own_pieces, opponent_pieces),
            Piece::Queen => queen_attacks(square, own_pieces, opponent_pieces),
            Piece::King => king_attacks(square, opponent_pieces),
        }
    }
}

/// Returns the moves a given pawn can make.
pub fn pawn_moves(square: Square, player: Player, blockers: BitBoard) -> BitBoard {
    let double_blockers = match player {
        Player::White => blockers | blockers << 8,
        Player::Black => blockers | blockers >> 8,
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
    fn forward_moves(square: Square, blockers: BitBoard) -> BitBoard {
        let file_mask = square.file().to_bitboard();
        let pot_blockers = blockers & file_mask;

        let difference = pot_blockers - BitBoard::new((Wrapping(square.to_bitboard().to_u64()) * Wrapping(2)).0);
        let changed = difference ^ blockers;
        changed & file_mask
    }

    let up = forward_moves(square, blockers);
    let down = forward_moves(square.mirror_horizontal(), blockers.mirror_horizontal()).mirror_horizontal();
    let left = forward_moves(square.mirror_diag(), blockers.mirror_diag()).mirror_diag();
    let right = forward_moves(
        square.mirror_diag().mirror_horizontal(),
        blockers.mirror_diag().mirror_horizontal(),
    ).mirror_horizontal()
        .mirror_diag();

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

fn bishop_all_moves(square: Square, blockers: BitBoard) -> BitBoard {
    // top-right diagonal only
    fn forward_moves(square: Square, blockers: BitBoard) -> BitBoard {
        let file_mask = square.diagonal();
        let pot_blockers = blockers & file_mask;

        let difference = pot_blockers - BitBoard::new((Wrapping(square.to_bitboard().to_u64()) * Wrapping(2)).0);
        let changed = difference ^ blockers;
        changed & file_mask
    }

    let forward_diag = forward_moves(square, blockers);
    let backward_diag = forward_moves(
        square.mirror_horizontal().mirror_diag().mirror_horizontal(),
        blockers
            .mirror_horizontal()
            .mirror_diag()
            .mirror_horizontal(),
    ).mirror_horizontal()
        .mirror_diag()
        .mirror_horizontal();

    let forward_antidiag = forward_moves(square.mirror_horizontal(), blockers.mirror_horizontal()).mirror_horizontal();
    let backward_antidiag = forward_moves(
        square.mirror_diag().mirror_horizontal(),
        blockers.mirror_diag().mirror_horizontal(),
    ).mirror_horizontal()
        .mirror_diag();

    forward_diag | forward_antidiag | backward_diag | backward_antidiag
}

/// Returns the moves a given bishop can make.
pub fn bishop_moves(square: Square, blockers: BitBoard) -> BitBoard {
    bishop_all_moves(square, blockers) & !blockers
}

/// Returns the attacks a given bishop can make.
pub fn bishop_attacks(square: Square, own_pieces: BitBoard, opponent_pieces: BitBoard) -> BitBoard {
    bishop_all_moves(square, own_pieces | opponent_pieces) & opponent_pieces
}

// -----------------------------------

/// Returns the moves a given queen can make.
pub fn queen_moves(square: Square, blockers: BitBoard) -> BitBoard {
    bishop_moves(square, blockers) | rook_moves(square, blockers)
}

/// Returns the attacks a given queen can make.
pub fn queen_attacks(square: Square, own_pieces: BitBoard, opponent_pieces: BitBoard) -> BitBoard {
    bishop_attacks(square, own_pieces, opponent_pieces) | rook_attacks(square, own_pieces, opponent_pieces)
}
