use board::player::Player;

#[test]
fn player() {
    let black = Player::White.other();
    let white = black.other();
    assert_eq!(black, Player::Black);
    assert_eq!(white, Player::White);
}
