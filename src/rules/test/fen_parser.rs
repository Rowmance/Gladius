use board::bitboard::BitBoard;
use board::file::File;
use board::player::Player;
use board::rank::Rank;
use board::square::Square;
use logger;
use rules::castle_rights::CastleRights;
use rules::fen_parser::parse_fen;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;

#[test]
fn parse_default() {
    let state = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    assert_eq!(state, GameState::start_position());
}

#[test]
fn parse_example1() {
    let state = parse_fen("5k2/ppp5/4P3/3R3p/6P1/1K2Nr2/PP3P2/8 b - - 1 32").unwrap();

    use board::file::File::*;
    use board::rank::Rank::*;
    let expected = GameState {
        white_board: PlayerBoard {
            pawns: BitBoard::empty()
                .set_coordinate(A, Two)
                .set_coordinate(B, Two)
                .set_coordinate(F, Two)
                .set_coordinate(G, Four)
                .set_coordinate(E, Six),
            rooks: BitBoard::empty().set_coordinate(D, Five),
            knights: BitBoard::empty().set_coordinate(E, Three),
            bishops: BitBoard::empty(),
            queens: BitBoard::empty(),
            king: BitBoard::empty().set_coordinate(B, Three),
        },
        black_board: PlayerBoard {
            pawns: BitBoard::empty()
                .set_coordinate(A, Seven)
                .set_coordinate(B, Seven)
                .set_coordinate(C, Seven)
                .set_coordinate(H, Five),
            rooks: BitBoard::empty().set_coordinate(F, Three),
            knights: BitBoard::empty(),
            bishops: BitBoard::empty(),
            queens: BitBoard::empty(),
            king: BitBoard::empty().set_coordinate(F, Eight),
        },
        player_turn: Player::Black,
        en_passant: None,
        white_castle_rights: CastleRights::None,
        black_castle_rights: CastleRights::None,
        draw_plies: 1,
        full_turns: 31,
    };

    assert_eq!(state, expected);
}

#[test]
fn parse_en_passant() {
    let state = parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    assert_eq!(
        state.en_passant,
        Some(Square::from_coordinates(File::E, Rank::Three))
    );
}
