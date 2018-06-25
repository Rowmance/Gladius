//! The alpha-beta tree-searching code.

use board::player::Player;
use engine::heuristic;
use rules::game_state::GameState;
use rules::move_application::Move;
use std::cmp;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

/// A sequence of turns with a given score.
///
/// The score given represents the engine's assertion of the position by the end of
/// the sequence.
#[derive(Debug)]
pub struct ScoredSequence {
    /// The sequence score in centipawns. Positive means white is winning.
    score: i32,

    /// Present if there's a forced mate.
    mate: Option<MateSummary>,

    /// The sequence of moves.
    moves: Vec<Move>,
}

impl Display for ScoredSequence {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let moves = self.moves
            .iter()
            .map(|m| format!("{}", m))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}: {}", self.score, moves)
    }
}

/// A summary of a mating sequence.
#[derive(Debug)]
pub struct MateSummary {
    /// The player who will mate.
    player: Player,

    /// The number of turns in which the player will mate.
    turns: u8,
}

// ---------------------------------------------------------------------
/// Returns the score of the given position using the alpha-beta algorithm.
fn alpha_beta_internal(state: &GameState, depth: usize, mut alpha: i32, mut beta: i32) -> i32 {
    // if mate, exit immediately
    if state.is_mate(Player::White) {
        return -200000;
    }
    if state.is_mate(Player::Black) {
        return 200000;
    }

    // if stalemate, also exit immediately
    if state.is_stale_mate(state.player_turn) {
        return 0;
    }

    if depth == 0 {
        return heuristic::score(&state);
    }

    let moves = state.legal_moves();
    if state.player_turn == Player::White {
        let mut max_eval = i32::min_value();
        for mv in moves {
            // TODO order move search checks + capturers -> checks normal -> captures -> normal
            let new_state = state.apply_move(&mv);
            let eval = alpha_beta_internal(&new_state, depth - 1, alpha, beta);
            //            println!("{}, {}: {}", mv, new_state, eval);
            max_eval = cmp::max(max_eval, eval);
            alpha = cmp::max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    }

    let mut min_eval = i32::max_value();
    for mv in moves {
        let new_state = state.apply_move(&mv);
        let eval = alpha_beta_internal(&new_state, depth - 1, alpha, beta);
        //        println!("{}, {}: {}", mv, new_state, eval);
        min_eval = cmp::min(min_eval, eval);
        beta = cmp::min(beta, eval);
        if beta <= alpha {
            break;
        }
    }
    return min_eval;
}

///
pub fn alpha_beta(state: &GameState, depth: usize) -> i32 {
    alpha_beta_internal(state, depth, i32::min_value(), i32::max_value())
}
