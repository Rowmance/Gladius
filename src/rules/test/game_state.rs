use rules::game_state::GameState;

#[test]
fn game_state() {
    let state = GameState::start_position(); // TODO, test the board.

    println!("{}", state);
}
