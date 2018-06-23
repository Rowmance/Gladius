use board::bitboard::BitBoard;
use board::file::File;
use board::piece::Piece;
use board::rank::Rank;
use board::square::Square;
use rules::castle_rights::CastleRights;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;

#[test]
fn starting_position() {
    let state = GameState::start_position();
    let all_moves = state.legal_moves();

    let pawn_moves: Vec<_> = all_moves
        .iter()
        .filter(|m| m.piece == Piece::Pawn)
        .collect();

    let knight_moves: Vec<_> = all_moves
        .iter()
        .filter(|m| m.piece == Piece::Knight)
        .collect();

    println!("{:?}", all_moves);

    assert_eq!(all_moves.len(), 20);
    assert_eq!(pawn_moves.len(), 16);
    assert_eq!(knight_moves.len(), 4);
}

#[test]
fn en_passant() {
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - . B - - - |
    // | - - - B W - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | K - - - - - - k |
    // +-+-+-+-+-+-+-+-+-+
    let state = GameState::default()
        .with_black_castle_rights(CastleRights::None)
        .with_white_castle_rights(CastleRights::None)
        .with_black_board(
            PlayerBoard::default()
                .with_pawns(
                    BitBoard::empty()
                        .set_coordinate(File::D, Rank::Five)
                        .set_coordinate(File::E, Rank::Six),
                )
                .with_king(BitBoard::empty().set_coordinate(File::A, Rank::One)),
        )
        .with_white_board(
            PlayerBoard::default()
                .with_pawns(BitBoard::empty().set_coordinate(File::E, Rank::Five))
                .with_king(BitBoard::empty().set_coordinate(File::H, Rank::One)),
        )
        .with_en_passant(Some(Square::from_coordinates(File::D, Rank::Six)));

    let moves = state.legal_moves();
    assert_eq!(moves.len(), 4); // capture + 3 king moves
    assert_eq!(moves.iter().any(|move_| move_.en_passant), true);
}

#[test]
fn promotion() {
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - - |
    // | - P - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | K - - - - - - k |
    // +-+-+-+-+-+-+-+-+-+
    let state = GameState::default()
        .with_black_castle_rights(CastleRights::None)
        .with_white_castle_rights(CastleRights::None)
        .with_white_board(
            PlayerBoard::default()
                .with_pawns(BitBoard::empty().set_coordinate(File::B, Rank::Seven))
                .with_king(BitBoard::empty().set_coordinate(File::A, Rank::One)),
        )
        .with_black_board(
            PlayerBoard::default().with_king(BitBoard::empty().set_coordinate(File::H, Rank::One)),
        )
        .with_en_passant(Some(Square::from_coordinates(File::D, Rank::Six)));

    let moves = state.legal_moves();
    println!("{:#?}", moves);
    assert_eq!(moves.len(), 7); // 4 promotions + 3 king moves

    let promo_moves: Vec<_> = moves
        .iter()
        .filter(|m| m.promotion.is_some())
        .filter(|m| m.target == Square::from_coordinates(File::B, Rank::Eight))
        .collect();

    assert_eq!(promo_moves.len(), 4);
}

#[test]
fn include_checks() {
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - r |
    // | - - - - - - - k |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | K - - - - - - - |
    // +-+-+-+-+-+-+-+-+-+
    let state = GameState::default()
        .with_black_castle_rights(CastleRights::None)
        .with_white_castle_rights(CastleRights::None)
        .with_white_board(
            PlayerBoard::default()
                .with_rooks(BitBoard::empty().set_coordinate(File::H, Rank::Eight))
                .with_king(BitBoard::empty().set_coordinate(File::H, Rank::Seven)),
        )
        .with_black_board(
            PlayerBoard::default().with_king(BitBoard::empty().set_coordinate(File::A, Rank::One)),
        );

    let moves = state.legal_moves();
    println!("{:?}", moves);
    assert_eq!(moves.len(), 11); // 4 king + 7 rook moves

    let rook_moves: Vec<_> = moves.iter().filter(|m| m.piece == Piece::Rook).collect();

    assert_eq!(rook_moves.len(), 7);
}

#[test]
fn exclude_self_checks() {
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - k |
    // | - - - - - - - b |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | K - - - - - - R |
    // +-+-+-+-+-+-+-+-+-+
    // cannot move bishop
    let state = GameState::default()
        .with_black_castle_rights(CastleRights::None)
        .with_white_castle_rights(CastleRights::None)
        .with_white_board(
            PlayerBoard::default()
                .with_bishops(BitBoard::empty().set_coordinate(File::H, Rank::Seven))
                .with_king(BitBoard::empty().set_coordinate(File::H, Rank::Eight)),
        )
        .with_black_board(
            PlayerBoard::default()
                .with_king(BitBoard::empty().set_coordinate(File::A, Rank::One))
                .with_rooks(BitBoard::empty().set_coordinate(File::H, Rank::One)),
        );

    let moves = state.legal_moves();
    println!("{:?}", moves);
    assert_eq!(moves.len(), 2); // 2 king moves

    let king_moves: Vec<_> = moves.iter().filter(|m| m.piece == Piece::King).collect();

    assert_eq!(king_moves.len(), 2);
}

#[test]
fn only_save_check() {
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - r |
    // | - - - - - - - k |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // | K - - - - - - R |
    // +-+-+-+-+-+-+-+-+-+
    // have to move king out of check
    let state = GameState::default()
        .with_black_castle_rights(CastleRights::None)
        .with_white_castle_rights(CastleRights::None)
        .with_white_board(
            PlayerBoard::default()
                .with_rooks(BitBoard::empty().set_coordinate(File::H, Rank::Eight))
                .with_king(BitBoard::empty().set_coordinate(File::H, Rank::Seven)),
        )
        .with_black_board(
            PlayerBoard::default()
                .with_king(BitBoard::empty().set_coordinate(File::A, Rank::One))
                .with_rooks(BitBoard::empty().set_coordinate(File::H, Rank::One)),
        );

    let moves = state.legal_moves();
    println!("{:?}", moves);
    assert_eq!(moves.len(), 3); // 3 king moves

    let king_moves: Vec<_> = moves.iter().filter(|m| m.piece == Piece::King).collect();

    assert_eq!(king_moves.len(), 3);
}

#[test]
fn complex_position() {
    // white and then black.
}

// TODO.
