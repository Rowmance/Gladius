use board::player::Player;
use rules::player_board::PlayerBoard;
use board::bitboard::BitBoard;
use board::piece::Piece;

#[test]
fn player_board() {
    let mut board = PlayerBoard::start_position(Player::White);

    assert_eq!(board.all(), BitBoard::new(65535));
    assert_eq!(board.piece(Piece::Pawn), BitBoard::new(65280));

    board = board.with_piece(Piece::Knight, BitBoard::new(1234));
    assert_eq!(board.knights, BitBoard::new(1234))
}
