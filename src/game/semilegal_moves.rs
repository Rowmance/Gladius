//! Generation of semi-legal moves.
//! These are moves which, unlike those in [basic_moves], consider the positions
//! of other pieces. These move generation functions do not consider check or checkmate.
//
//use board::square::Square;
//use board::piece::Piece;
//use board::player::Player;
//use board::rank::Rank;
//use board::file::File;
//use board::bitboard::BitBoard;
//use game::basic_moves;
//
///// Returns the moves for a pawn given a square and black/white pieces.
//pub fn pawn_moves(
//    square: Square,
//    player: Player,
//    own_pieces: BitBoard,
//    opponent_pieces: BitBoard
//) -> BitBoard {
//    let blockers = own_pieces | opponent_pieces;
//    let double_blockers = match player {
//        Player::White => blockers | blockers << 8,
//        Player::Black => blockers | blockers >> 8
//    };
//    basic_moves::pawn_moves(square, player) & !double_blockers
//}
//
///// Returns the attacks a given pawn can make.
//pub fn pawn_attacks(square: Square, player: Player, opponent_pieces: BitBoard) -> BitBoard {
//    basic_moves::pawn_attacks(square, player) & opponent_pieces
//}
