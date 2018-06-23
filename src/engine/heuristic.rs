//! The heuristic function used for the engine.

use board::bitboard::BitBoard;
use board::file::File;
use board::rank::Rank;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;
use std::i32;

/// Quickly computes the score of the given game state, in centipawns.
/// Positive values indicate that white is winning, negative values indicate that
/// black is winning. 0 indicates a drawn position.
/// A marginal score of 100 roughly indicates an advantage of a pawn.
pub fn score(state: &GameState) -> i32 {
    pawns(state) + knights(state) + bishops(state) + rooks(state) + queens(state) + king(state)
        + mobility(state) + defended_pieces(state)
}

// ---------------------------------------------------------------------
fn pawns(state: &GameState) -> i32 {
    let diff: i32 = state.white_board.pawns.count() as i32 - state.black_board.pawns.count() as i32;
    let material = diff * 100;

    // penalties for doubled pawns
    let black_doubled = File::iter()
        .map(|file| file.to_bitboard())
        .map(|bb| bb & state.black_board.pawns)
        .filter(|bb| bb.count() > 1)
        .count() as i32;

    let white_doubled = File::iter()
        .map(|file| file.to_bitboard())
        .map(|bb| bb & state.white_board.pawns)
        .filter(|bb| bb.count() > 1)
        .count() as i32;

    // penalties for isolated pawns
    let black_isolated = File::iter()
        .filter(|file| {
            let pawns_on_file = state.black_board.pawns & file.to_bitboard();
            !pawns_on_file.is_empty()
        })
        .map(|file| {
            let next = file.next()
                .map_or(BitBoard::empty(), |file| file.to_bitboard());
            let prev = file.prev()
                .map_or(BitBoard::empty(), |file| file.to_bitboard());
            next | prev
        })
        .filter(|&bb| {
            let adjacent_pawns = state.black_board.pawns & bb;
            adjacent_pawns.is_empty()
        })
        .count() as i32;

    let white_isolated = File::iter()
        .filter(|file| {
            let pawns_on_file = state.white_board.pawns & file.to_bitboard();
            !pawns_on_file.is_empty()
        })
        .map(|file| {
            let next = file.next()
                .map_or(BitBoard::empty(), |file| file.to_bitboard());
            let prev = file.prev()
                .map_or(BitBoard::empty(), |file| file.to_bitboard());
            next | prev
        })
        .filter(|&bb| {
            let adjacent_pawns = state.white_board.pawns & bb;
            adjacent_pawns.is_empty()
        })
        .count() as i32;

    // position mask
    static pawn_mask: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
        5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
        -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let white_mask = apply_mask(state.white_board.pawns, &pawn_mask);
    let black_mask = apply_mask_flipped(state.black_board.pawns, &pawn_mask);

    material + (black_doubled * 50) + (white_doubled * -50) + (black_isolated * 50)
        + (white_isolated * -50) + white_mask - black_mask

    // TODO
    // bonus for chain and center control
    // penalty for no pawns
}

fn knights(state: &GameState) -> i32 {
    let diff: i32 =
        state.white_board.knights.count() as i32 - state.black_board.knights.count() as i32;
    let material = diff * 300;

    // position mask
    static knight_mask: [i32; 64] = [
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ];

    let white_mask = apply_mask(state.white_board.knights, &knight_mask);
    let black_mask = apply_mask_flipped(state.black_board.knights, &knight_mask);

    material + white_mask - black_mask
    // decrease in value if fewer pawns
    // knight pair penalty (second knight is worth less)
}

fn bishops(state: &GameState) -> i32 {
    let diff: i32 =
        state.white_board.bishops.count() as i32 - state.black_board.bishops.count() as i32;
    let material = diff * 300;

    // position mask
    static bishop_mask: [i32; 64] = [
        -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5,
        0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10,
        10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
    ];

    let white_mask = apply_mask(state.white_board.bishops, &bishop_mask);
    let black_mask = apply_mask_flipped(state.black_board.bishops, &bishop_mask);

    material + white_mask - black_mask
    // bonus for bishop pair
    // penalty for bad bishop
    //
}

fn rooks(state: &GameState) -> i32 {
    let diff: i32 = state.white_board.rooks.count() as i32 - state.black_board.rooks.count() as i32;
    let material = diff * 500;

    // position mask
    static rook_mask: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
        0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0,
        -5, 0, 0, 0, 5, 5, 0, 0, 0,
    ];

    let white_mask = apply_mask(state.white_board.rooks, &rook_mask);
    let black_mask = apply_mask_flipped(state.black_board.rooks, &rook_mask);

    material + white_mask - black_mask
    // penalty for rook pair
    // bonus as pawns disappear
    // bonus for open file
    // bonus for 7th rank
    // bonus if enemy queen on same file
    // bonus for connected rooks
}

fn queens(state: &GameState) -> i32 {
    let diff: i32 =
        state.white_board.queens.count() as i32 - state.black_board.queens.count() as i32;
    let material = diff * 900;

    // position mask
    static queen_mask: [i32; 64] = [
        -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0,
        -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0,
        5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
    ];

    let white_mask = apply_mask(state.white_board.queens, &queen_mask);
    let black_mask = apply_mask_flipped(state.black_board.queens, &queen_mask);

    material + white_mask - black_mask
    // penalty for early development
}

fn king(state: &GameState) -> i32 {
    // position mask
    static king_mask: [i32; 64] = [
        -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
        -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
        -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
        30, 10, 0, 0, 10, 30, 20,
    ];

    static king_late_mask: [i32; 64] = [
        -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30, -10, 20,
        30, 30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30, 40, 40, 30, -10,
        -30, -30, -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0, -30, -30, -50, -30, -30,
        -30, -30, -30, -30, -50,
    ];

    let white_mask = apply_mask(state.white_board.king, &king_mask);
    let black_mask = apply_mask_flipped(state.black_board.king, &king_mask);

    white_mask - black_mask
    // king should be safe in early/mid game
    // king should be active late game
    // castling should be encouraged in this method
}

// ---------------------------------------------------------------------
fn mobility(_state: &GameState) -> i32 {
    0
    // +0.1 for each legal move
    // trapped pieces?
    // mobility for knights/bishops is more important early
}

fn defended_pieces(_state: &GameState) -> i32 {
    0
    // penalty for undefended minor piece
}

// ---------------------------------------------------------------------
/// Applies a mask to the given bitboard.
///
/// The mask is given as an array of ranks from 8 to 1, ie
/// ```
/// [A8, B8, ... H8,
/// A7, B7, ... H7,
/// ...
/// ]
/// ```
fn apply_mask_flipped(bitboard: BitBoard, mask: &[i32; 64]) -> i32 {
    let mut iterator = bitboard.to_u64();
    let mut sum: i32 = 0;
    while iterator > 0 {
        let index = iterator.trailing_zeros() as usize;
        sum += mask[index];
        iterator ^= (1 << index);
    }
    sum
}

// the mask is flipped by default.
/// Applies a mask to the given bitboard.
pub fn apply_mask(bitboard: BitBoard, mask: &[i32; 64]) -> i32 {
    apply_mask_flipped(bitboard.mirror_horizontal(), mask)
}
