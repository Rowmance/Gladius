use board::bitboard::BitBoard;
use board::bitboards;

use board::rank::Rank;

#[test]
fn bitboard_ops() {
    // Symmetrical
    assert!(bitboards::WHITE_START_PAWNS.flip() == bitboards::BLACK_START_PAWNS);
    assert!(bitboards::WHITE_START_ROOKS == bitboards::BLACK_START_ROOKS.flip());

    assert!(bitboards::WHITE_START_KNIGHTS.flip().flip() == bitboards::BLACK_START_KNIGHTS.flip());
    assert!(bitboards::WHITE_START_BISHOPS.flip() == bitboards::BLACK_START_BISHOPS.flip().flip());

    // King/Queen are mirrored
    assert!(bitboards::WHITE_START_QUEENS == bitboards::BLACK_START_KINGS.flip());
    assert!(bitboards::WHITE_START_KINGS.flip() == bitboards::BLACK_START_QUEENS);

    // Test flipping; xor, and, or operators
    let all_white = bitboards::WHITE_START_PAWNS
        | bitboards::WHITE_START_ROOKS
        | bitboards::WHITE_START_KNIGHTS
        | bitboards::WHITE_START_BISHOPS
        | bitboards::WHITE_START_QUEENS
        | bitboards::WHITE_START_KINGS;

    let white_pieces = all_white ^ bitboards::WHITE_START_PAWNS;
    assert!(white_pieces & bitboards::WHITE_START_PAWNS == bitboards::EMPTY);
    assert!(white_pieces & all_white == white_pieces);

    let black_pieces = all_white.flip() ^ bitboards::BLACK_START_PAWNS;
    assert!(black_pieces.flip() == white_pieces);

    let all_black = black_pieces | bitboards::BLACK_START_PAWNS;
    assert!(all_white.flip() == all_black);

    // Test not operator
    assert!(!!bitboards::BLACK_START_ROOKS ^ bitboards::BLACK_START_ROOKS == bitboards::EMPTY);
}

#[test]
fn rank() {
    assert!(Rank::TWO.to_index() == 1);
    assert!(Rank::from_index(6) == Rank::SEVEN);

    assert!(Rank::FOUR.to_bitboard() == BitBoard(578721382704613384));
    assert!(Rank::ONE.to_bitboard() == BitBoard(72340172838076673));
    assert!(Rank::EIGHT.to_bitboard() == BitBoard(9259542123273814144));
}
