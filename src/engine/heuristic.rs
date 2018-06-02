//! The heuristic function used for the engine.

use board::piece::Piece;
use board::player::Player;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;
use std::i64;

/// Quickly computes the score of the given game state, in centipawns.
/// Positive values indicate that white is winning, negative values indicate that
/// black is winning. 0 indicates a drawn position.
/// A marginal score of 100 roughly indicates an advantage of a pawn.
pub fn score(state: GameState) -> i64 {
    board_score(state.white_board) - board_score(state.black_board)
}

fn board_score(board: PlayerBoard) -> i64 {
    (board.pawns.count() * 100 + board.bishops.count() * 300 + board.knights.count() * 300
        + board.rooks.count() * 500 + board.queens.count() * 900) as i64
}
