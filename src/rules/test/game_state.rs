use rules::game_state::GameState;
use rules::player_board::PlayerBoard;
use board::player::Player;
use rules::castle_rights::CastleRights;

#[test]
fn game_state() {
    let mut state = GameState::start_position();

    assert_eq!(
        state.player_board(Player::White),
        PlayerBoard::start_position(Player::White)
    );
    assert_eq!(
        state.player_board(Player::Black),
        PlayerBoard::start_position(Player::Black)
    );

    state = state.with_white_castle_rights(CastleRights::QueenSide);
    assert_eq!(state.castle_rights(Player::White), CastleRights::QueenSide);
    assert_eq!(state.castle_rights(Player::Black), CastleRights::Both);

    state.set_castle_rights(Player::Black, CastleRights::KingSide);
    state.set_castle_rights(Player::White, CastleRights::KingSide);
    assert_eq!(state.black_castle_rights, CastleRights::KingSide);
    assert_eq!(state.white_castle_rights, CastleRights::KingSide);
}
