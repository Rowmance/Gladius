//! Tests for the board module.

use board::bitboard::BitBoard;
use board::bitboards;
use board::square::Square;
use board::rank::Rank;
use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::board::Board;

use std::panic::catch_unwind;

//TODO: Use assert_eq for equality assertions.

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

    // Test iterator
    let mut count = 0;
    for sq in bitboards::BLACK_START_PAWNS.iter() {
        assert!(sq.rank() == Rank::Seven);
        count += 1;
    }
    assert!(count == 8)
}


#[test]
fn square() {
    assert!(Square::new(0) == Square::from_coordinates(File::A, Rank::One));
    assert!(Square::new(3) == Square::from_coordinates(File::D, Rank::One));
    assert!(Square::new(7) == Square::from_coordinates(File::H, Rank::One));
    assert!(Square::new(8) == Square::from_coordinates(File::A, Rank::Two));
    assert!(Square::new(27) == Square::from_coordinates(File::D, Rank::Four));

    assert!(Square::new(47) == Square::from_coordinates(File::H, Rank::Six));
    assert!(Square::new(63) == Square::from_coordinates(File::H, Rank::Eight));

    let square = Square::from_coordinates(File::C, Rank::Two);
    assert!(square.file() == File::C);
    assert!(square.rank() == Rank::Two);

    debug_assert!(catch_unwind(|| Square::new(64)).is_err());
}

#[test]
fn rank() {
    assert!(Rank::Two.to_index() == 1);
    assert!(Rank::from_index(6) == Rank::Seven);

    assert!(Rank::Four.to_bitboard() == BitBoard(578721382704613384));
    assert!(Rank::One.to_bitboard() == BitBoard(72340172838076673));
    assert!(Rank::Eight.to_bitboard() == BitBoard(9259542123273814144));
}

#[test]
fn file() {
    assert!(File::B.to_index() == 1);
    assert!(File::from_index(6) == File::G);

    assert!(File::A.to_bitboard() == BitBoard(255));
    assert!(File::D.to_bitboard() == BitBoard(4278190080));
    assert!(File::H.to_bitboard() == BitBoard(18374686479671623680));
}

#[test]
fn board() {
    let board = Board::start_position();

    println!("{}", board);

    //assert!(false)
}