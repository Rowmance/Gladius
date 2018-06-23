use board::bitboard;
use board::bitboard::BitBoard;
use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::square::Square;
use rules::castle_rights::CastleRights;
use rules::game_state::GameState;
use rules::move_application::CastleMove;
use rules::move_application::Move;
use rules::player_board::PlayerBoard;

#[test]
fn move_castle_white() {
    let initial_state = GameState::default()
        .with_white_castle_rights(CastleRights::Both)
        .with_black_castle_rights(CastleRights::Both)
        .with_player_turn(Player::White)
        .with_white_board(
            PlayerBoard::default()
                .with_rooks(bitboard::WHITE_START_ROOKS)
                .with_king(bitboard::WHITE_START_KINGS),
        )
        .with_black_board(
            PlayerBoard::default()
                .with_rooks(bitboard::BLACK_START_ROOKS)
                .with_king(bitboard::BLACK_START_KINGS),
        );

    let ks_move = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::One),
        target: Square::from_coordinates(File::G, Rank::One),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: Some(CastleMove::KingSide),
    };

    let ks_state = initial_state.apply_move(&ks_move);

    println!("{}", initial_state);
    println!("{}", ks_state);

    assert_eq!(
        ks_state,
        GameState {
            white_board: PlayerBoard::default()
                .with_rooks(
                    BitBoard::empty()
                        .set_coordinate(File::A, Rank::One)
                        .set_coordinate(File::F, Rank::One)
                )
                .with_king(BitBoard::empty().set_coordinate(File::G, Rank::One)),
            black_board: PlayerBoard::default()
                .with_rooks(bitboard::BLACK_START_ROOKS)
                .with_king(bitboard::BLACK_START_KINGS),
            player_turn: Player::Black,
            en_passant: None,
            white_castle_rights: CastleRights::None,
            black_castle_rights: CastleRights::Both,
            draw_plies: 1,
            full_turns: 0,
        }
    );

    // ------
    let qs_move = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::One),
        target: Square::from_coordinates(File::C, Rank::One),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: Some(CastleMove::QueenSide),
    };

    let qs_state = initial_state.apply_move(&qs_move);

    println!("{}", initial_state);
    println!("{}", qs_state);

    assert_eq!(
        qs_state,
        GameState {
            white_board: PlayerBoard::default()
                .with_rooks(
                    BitBoard::empty()
                        .set_coordinate(File::H, Rank::One)
                        .set_coordinate(File::D, Rank::One)
                )
                .with_king(BitBoard::empty().set_coordinate(File::C, Rank::One)),
            black_board: PlayerBoard::default()
                .with_rooks(bitboard::BLACK_START_ROOKS)
                .with_king(bitboard::BLACK_START_KINGS),
            player_turn: Player::Black,
            en_passant: None,
            white_castle_rights: CastleRights::None,
            black_castle_rights: CastleRights::Both,
            draw_plies: 1,
            full_turns: 0,
        }
    );
}

#[test]
fn move_castle_black() {
    let initial_state = GameState::default()
        .with_white_castle_rights(CastleRights::Both)
        .with_black_castle_rights(CastleRights::Both)
        .with_player_turn(Player::Black)
        .with_white_board(
            PlayerBoard::default()
                .with_rooks(bitboard::WHITE_START_ROOKS)
                .with_king(bitboard::WHITE_START_KINGS),
        )
        .with_black_board(
            PlayerBoard::default()
                .with_rooks(bitboard::BLACK_START_ROOKS)
                .with_king(bitboard::BLACK_START_KINGS),
        );

    let ks_move = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::Eight),
        target: Square::from_coordinates(File::G, Rank::Eight),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: Some(CastleMove::KingSide),
    };

    let ks_state = initial_state.apply_move(&ks_move);

    println!("{}", initial_state);
    println!("{}", ks_state);

    assert_eq!(
        ks_state,
        GameState {
            white_board: PlayerBoard::default()
                .with_rooks(bitboard::WHITE_START_ROOKS)
                .with_king(bitboard::WHITE_START_KINGS),
            black_board: PlayerBoard::default()
                .with_rooks(
                    BitBoard::empty()
                        .set_coordinate(File::A, Rank::Eight)
                        .set_coordinate(File::F, Rank::Eight)
                )
                .with_king(BitBoard::empty().set_coordinate(File::G, Rank::Eight)),
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::None,
            draw_plies: 1,
            full_turns: 1,
        }
    );

    // ------
    let qs_move = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::Eight),
        target: Square::from_coordinates(File::C, Rank::Eight),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: Some(CastleMove::QueenSide),
    };

    let qs_state = initial_state.apply_move(&qs_move);

    println!("{}", initial_state);
    println!("{}", qs_state);

    assert_eq!(
        qs_state,
        GameState {
            white_board: PlayerBoard::default()
                .with_rooks(bitboard::WHITE_START_ROOKS)
                .with_king(bitboard::WHITE_START_KINGS),
            black_board: PlayerBoard::default()
                .with_rooks(
                    BitBoard::empty()
                        .set_coordinate(File::H, Rank::Eight)
                        .set_coordinate(File::D, Rank::Eight)
                )
                .with_king(BitBoard::empty().set_coordinate(File::C, Rank::Eight)),
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::None,
            draw_plies: 1,
            full_turns: 1,
        }
    );
}

#[test]
fn move_promote_white() {
    let initial_state = GameState::default()
        .with_player_turn(Player::White)
        .with_white_board(
            PlayerBoard::default()
                .with_pawns(BitBoard::empty().set_coordinate(File::G, Rank::Seven)),
        );

    let move_ = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::G, Rank::Seven),
        target: Square::from_coordinates(File::G, Rank::Eight),
        capture: false,
        en_passant: false,
        promotion: Some(Piece::Queen),
        castle: None,
    };

    let state = initial_state.apply_move(&move_);

    println!("{}", initial_state);
    println!("{}", state);

    assert_eq!(
        state,
        GameState {
            white_board: PlayerBoard::default()
                .with_queens(BitBoard::empty().set_coordinate(File::G, Rank::Eight)),
            black_board: PlayerBoard::default(),
            player_turn: Player::Black,
            en_passant: None,
            white_castle_rights: CastleRights::None,
            black_castle_rights: CastleRights::None,
            draw_plies: 0,
            full_turns: 0,
        }
    );
}

#[test]
fn move_promote_black() {
    let initial_state = GameState::default()
        .with_player_turn(Player::Black)
        .with_black_board(
            PlayerBoard::default().with_pawns(BitBoard::empty().set_coordinate(File::B, Rank::Two)),
        );

    let move_ = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::B, Rank::Two),
        target: Square::from_coordinates(File::B, Rank::One),
        capture: false,
        en_passant: false,
        promotion: Some(Piece::Knight),
        castle: None,
    };

    let state = initial_state.apply_move(&move_);

    println!("{}", initial_state);
    println!("{}", state);

    assert_eq!(
        state,
        GameState {
            white_board: PlayerBoard::default(),
            black_board: PlayerBoard::default()
                .with_knights(BitBoard::empty().set_coordinate(File::B, Rank::One)),
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::None,
            black_castle_rights: CastleRights::None,
            draw_plies: 0,
            full_turns: 1,
        }
    );
}

#[test]
fn move_sequence() {
    let mut state = GameState::start_position();
    println!("{}", state);

    let move1 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::D, Rank::Two),
        target: Square::from_coordinates(File::D, Rank::Four),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move1);
    println!("{}", state);

    let move2 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::D, Rank::Seven),
        target: Square::from_coordinates(File::D, Rank::Five),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move2);
    println!("{}", state);

    let move3 = Move {
        piece: Piece::Bishop,
        origin: Square::from_coordinates(File::C, Rank::One),
        target: Square::from_coordinates(File::F, Rank::Four),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move3);
    println!("{}", state);

    let move4 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::E, Rank::Seven),
        target: Square::from_coordinates(File::E, Rank::Five),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move4);
    println!("{}", state);

    assert_eq!(
        state,
        GameState {
            white_board: PlayerBoard {
                pawns: BitBoard::new(134280960),
                rooks: BitBoard::new(129),
                knights: BitBoard::new(66),
                bishops: BitBoard::new(536870944),
                queens: BitBoard::new(8),
                king: BitBoard::new(16),
            },
            black_board: PlayerBoard {
                pawns: BitBoard::new(65020822699376640),
                rooks: BitBoard::new(9295429630892703744),
                knights: BitBoard::new(4755801206503243776),
                bishops: BitBoard::new(2594073385365405696),
                queens: BitBoard::new(576460752303423488),
                king: BitBoard::new(1152921504606846976),
            },
            player_turn: Player::White,
            en_passant: Some(Square::new(44)),
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::Both,
            draw_plies: 0,
            full_turns: 2,
        }
    );
}

#[test]
fn attack_sequence() {
    let mut state = GameState::start_position();
    println!("{}", state);

    let move1 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::E, Rank::Two),
        target: Square::from_coordinates(File::E, Rank::Four),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move1);
    println!("{}", state);

    let move2 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::D, Rank::Seven),
        target: Square::from_coordinates(File::D, Rank::Five),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move2);
    println!("{}", state);

    let move3 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::E, Rank::Four),
        target: Square::from_coordinates(File::D, Rank::Five),
        capture: true,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move3);
    println!("{}", state);

    let move4 = Move {
        piece: Piece::Queen,
        origin: Square::from_coordinates(File::D, Rank::Eight),
        target: Square::from_coordinates(File::D, Rank::Five),
        capture: true,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move4);
    println!("{}", state);

    assert_eq!(
        state,
        GameState {
            white_board: PlayerBoard {
                pawns: BitBoard::new(61184),
                rooks: BitBoard::new(129),
                knights: BitBoard::new(66),
                bishops: BitBoard::new(36),
                queens: BitBoard::new(8),
                king: BitBoard::new(16),
            },
            black_board: PlayerBoard {
                pawns: BitBoard::new(69524319247532032),
                rooks: BitBoard::new(9295429630892703744),
                knights: BitBoard::new(4755801206503243776),
                bishops: BitBoard::new(2594073385365405696),
                queens: BitBoard::new(34359738368),
                king: BitBoard::new(1152921504606846976),
            },
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::Both,
            draw_plies: 0,
            full_turns: 2,
        }
    );
}

#[test]
fn en_passant() {
    let mut state = GameState::start_position().with_black_board(
        PlayerBoard::default()
            .with_pawns(Square::from_coordinates(File::E, Rank::Four).to_bitboard()),
    );
    println!("{}", state);

    let move1 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::D, Rank::Two),
        target: Square::from_coordinates(File::D, Rank::Four),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move1);
    println!("{}", state);

    let move2 = Move {
        piece: Piece::Pawn,
        origin: Square::from_coordinates(File::E, Rank::Four),
        target: Square::from_coordinates(File::D, Rank::Three),
        capture: true,
        en_passant: true,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move2);
    println!("{}", state);

    assert_eq!(
        state,
        GameState {
            white_board: PlayerBoard {
                pawns: BitBoard::new(63232),
                rooks: BitBoard::new(129),
                knights: BitBoard::new(66),
                bishops: BitBoard::new(36),
                queens: BitBoard::new(8),
                king: BitBoard::new(16),
            },
            black_board: PlayerBoard {
                pawns: BitBoard::new(524288),
                rooks: BitBoard::new(0),
                knights: BitBoard::new(0),
                bishops: BitBoard::new(0),
                queens: BitBoard::new(0),
                king: BitBoard::new(0),
            },
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::Both,
            draw_plies: 0,
            full_turns: 1,
        }
    );
}

#[test]
fn castle_rights_set_on_move() {
    let mut state = GameState::default()
        .with_white_castle_rights(CastleRights::Both)
        .with_black_castle_rights(CastleRights::Both)
        .with_white_board(PlayerBoard::start_position(Player::White).with_pawns(BitBoard::empty()))
        .with_black_board(PlayerBoard::start_position(Player::Black).with_pawns(BitBoard::empty()));

    let move1 = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::One),
        target: Square::from_coordinates(File::D, Rank::Two),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move1);
    assert_eq!(state.white_castle_rights, CastleRights::None);
    assert_eq!(state.black_castle_rights, CastleRights::Both);

    let move2 = Move {
        piece: Piece::Rook,
        origin: Square::from_coordinates(File::A, Rank::Eight),
        target: Square::from_coordinates(File::A, Rank::Three),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: None,
    };

    state = state.apply_move(&move2);
    assert_eq!(state.white_castle_rights, CastleRights::None);
    assert_eq!(state.black_castle_rights, CastleRights::KingSide);
}

#[test]
fn castle_rights_set_on_castle() {
    let mut state = GameState::default()
        .with_white_castle_rights(CastleRights::Both)
        .with_black_castle_rights(CastleRights::Both)
        .with_white_board(
            PlayerBoard::start_position(Player::White)
                .with_pawns(BitBoard::empty())
                .with_knights(BitBoard::empty())
                .with_bishops(BitBoard::empty()),
        )
        .with_black_board(
            PlayerBoard::start_position(Player::Black)
                .with_pawns(BitBoard::empty())
                .with_knights(BitBoard::empty())
                .with_bishops(BitBoard::empty())
                .with_queens(BitBoard::empty()),
        );

    let move1 = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::One),
        target: Square::from_coordinates(File::G, Rank::One),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: Some(CastleMove::KingSide),
    };

    state = state.apply_move(&move1);
    assert_eq!(state.white_castle_rights, CastleRights::None);
    assert_eq!(state.black_castle_rights, CastleRights::Both);

    let move2 = Move {
        piece: Piece::King,
        origin: Square::from_coordinates(File::E, Rank::Eight),
        target: Square::from_coordinates(File::B, Rank::Eight),
        capture: false,
        en_passant: false,
        promotion: None,
        castle: Some(CastleMove::QueenSide),
    };

    state = state.apply_move(&move2);
    assert_eq!(state.white_castle_rights, CastleRights::None);
    assert_eq!(state.black_castle_rights, CastleRights::None);
}
