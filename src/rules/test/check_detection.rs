use rules::game_state::GameState;
use rules::player_board::PlayerBoard;
use board::square::Square;
use board::file::File;
use board::rank::Rank;

#[test]
fn is_check_rook() {
    let state = GameState::default()
        .with_white_board(PlayerBoard::default().with_king(Square::from_coordinates(File::B, Rank::Two).to_bitboard()))
        .with_black_board(
            PlayerBoard::default().with_rooks(Square::from_coordinates(File::B, Rank::Four).to_bitboard()),
        );

    println!("{}", state);
    assert_eq!(state.is_check(), true);

    let state = state.clone().with_white_board(
        state
            .white_board
            .with_pawns(Square::from_coordinates(File::B, Rank::Three).to_bitboard()),
    );

    println!("{}", state);
    assert_eq!(state.is_check(), false);
}

#[test]
fn is_check_pawn() {
    let state = GameState::default()
        .with_white_board(PlayerBoard::default().with_king(Square::from_coordinates(File::B, Rank::Two).to_bitboard()))
        .with_black_board(
            PlayerBoard::default().with_pawns(Square::from_coordinates(File::C, Rank::Three).to_bitboard()),
        );

    println!("{}", state);
    assert_eq!(state.is_check(), true);
}

#[test]
fn is_check_knight() {
    let state = GameState::default()
        .with_white_board(PlayerBoard::default().with_king(Square::from_coordinates(File::B, Rank::Two).to_bitboard()))
        .with_black_board(
            PlayerBoard::default().with_knights(Square::from_coordinates(File::C, Rank::Four).to_bitboard()),
        );

    println!("{}", state);
    assert_eq!(state.is_check(), true);
}

#[test]
fn is_check_bishop() {
    let state = GameState::default()
        .with_white_board(PlayerBoard::default().with_king(Square::from_coordinates(File::B, Rank::Two).to_bitboard()))
        .with_black_board(
            PlayerBoard::default().with_bishops(Square::from_coordinates(File::E, Rank::Five).to_bitboard()),
        );

    println!("{}", state);
    assert_eq!(state.is_check(), true);

    let state = state.clone().with_white_board(
        state
            .white_board
            .with_pawns(Square::from_coordinates(File::C, Rank::Three).to_bitboard()),
    );

    println!("{}", state);
    assert_eq!(state.is_check(), false);
}

// TODO: Add mate tests
