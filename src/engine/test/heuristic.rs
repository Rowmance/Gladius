use board::bitboard::BitBoard;
use board::file::File;
use board::rank::Rank;
use engine::heuristic;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;

#[test]
pub fn heuristic_start() {
    let state = GameState::start_position();
    assert_eq!(heuristic::score(state), 0);

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

    assert!(heuristic::score(state2) < 0);
}
