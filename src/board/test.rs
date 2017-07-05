use board::bitboard::BitBoard;
use board::bitboards;
use board::square::Square;
use board::rank::Rank;
use board::file::File;

use std::panic::catch_unwind;

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
fn square() {
    assert!(Square::new(0) == Square::from_coordinates(File::A, Rank::ONE));
    assert!(Square::new(3) == Square::from_coordinates(File::D, Rank::ONE));
    assert!(Square::new(7) == Square::from_coordinates(File::H, Rank::ONE));
    assert!(Square::new(8) == Square::from_coordinates(File::A, Rank::TWO));
    assert!(Square::new(27) == Square::from_coordinates(File::D, Rank::FOUR));

    assert!(Square::new(47) == Square::from_coordinates(File::H, Rank::SIX));
    assert!(Square::new(63) == Square::from_coordinates(File::H, Rank::EIGHT));

    let square = Square::from_coordinates(File::C, Rank::TWO);
    assert!(square.file() == File::C);
    assert!(square.rank() == Rank::TWO);

    debug_assert!(catch_unwind(|| Square::new(64)).is_err());
}

#[test]
fn rank() {
    assert!(Rank::TWO.to_index() == 1);
    assert!(Rank::from_index(6) == Rank::SEVEN);

    assert!(Rank::FOUR.to_bitboard() == BitBoard(578721382704613384));
    assert!(Rank::ONE.to_bitboard() == BitBoard(72340172838076673));
    assert!(Rank::EIGHT.to_bitboard() == BitBoard(9259542123273814144));
}

#[test]
fn file() {
    assert!(File::B.to_index() == 1);
    assert!(File::from_index(6) == File::G);

    assert!(File::A.to_bitboard() == BitBoard(255));
    assert!(File::D.to_bitboard() == BitBoard(4278190080));
    assert!(File::H.to_bitboard() == BitBoard(18374686479671623680));
}
