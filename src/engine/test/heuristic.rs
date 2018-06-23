use board::bitboard;
use board::bitboard::BitBoard;
use board::file::File;
use board::rank::Rank;
use engine::heuristic;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;
use std::vec::Vec;

#[test]
pub fn heuristic_start() {
    let state = GameState::start_position();
    assert_eq!(heuristic::score(&state), 0);

    let state2 = GameState::default()
        .with_white_board(
            PlayerBoard::default()
                .with_rooks(BitBoard::empty().set_coordinate(File::H, Rank::Eight))
                .with_king(BitBoard::empty().set_coordinate(File::H, Rank::Seven)),
        )
        .with_black_board(
            PlayerBoard::default()
                .with_king(BitBoard::empty().set_coordinate(File::A, Rank::One))
                .with_rooks(BitBoard::empty().set_coordinate(File::H, Rank::One))
                .with_bishops(BitBoard::empty().set_coordinate(File::A, Rank::Three)),
        );

    assert!(heuristic::score(&state2) < 0);
}

#[test]
pub fn apply_mask_white_pawns() {
    let mut mask: [i32; 64] = [0; 64];
    for i in 0..64 {
        mask[i] = i as i32;
    }

    let bb = bitboard::WHITE_START_PAWNS;
    let result = heuristic::apply_mask(bb, &mask);
    let expected = 412;
    assert_eq!(result, expected);
}

#[test]
pub fn apply_mask_black_pawns() {
    let mut mask: [i32; 64] = [0; 64];
    for i in 0..64 {
        mask[i] = i as i32;
    }

    let bb = bitboard::BLACK_START_PAWNS;
    let result = heuristic::apply_mask(bb, &mask);
    let expected = 92;
    assert_eq!(result, expected);
}

#[test]
pub fn apply_mask_complex() {
    let mut mask: [i32; 64] = [0; 64];
    for i in 0..64 {
        mask[i] = (i + 1) as i32;
    }

    let bb = bitboard::BitBoard::new(85937205623);
    println!("{}", bb);
    let result = heuristic::apply_mask(bb, &mask);
    let expected = 27 + 29 + 34 + 41 + 47 + 49 + 53 + 54 + 56 + 57 + 58 + 59 + 61 + 62 + 63;
    assert_eq!(result, expected);
}
