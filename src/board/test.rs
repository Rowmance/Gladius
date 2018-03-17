//! Tests for the board module.

use board::bitboard::BitBoard;
use board::bitboards;
use board::square::Square;
use board::rank::Rank;
use board::file::File;
use board::game_state::GameState;

use std::panic::catch_unwind;

#[test]
fn bitboard_ops() {
    // xor, and, or operators
    let all_white = bitboards::WHITE_START_PAWNS | bitboards::WHITE_START_ROOKS | bitboards::WHITE_START_KNIGHTS
        | bitboards::WHITE_START_BISHOPS | bitboards::WHITE_START_QUEENS
        | bitboards::WHITE_START_KINGS;

    let white_pieces = all_white ^ bitboards::WHITE_START_PAWNS;
    assert_eq!(
        white_pieces & bitboards::WHITE_START_PAWNS,
        BitBoard::empty()
    );
    assert_eq!(white_pieces & all_white, white_pieces);

    let black_pieces = all_white.flip() ^ bitboards::BLACK_START_PAWNS;
    assert_eq!(black_pieces.flip(), white_pieces);

    let all_black = black_pieces | bitboards::BLACK_START_PAWNS;
    assert_eq!(all_white.flip(), all_black);

    // not operator
    assert_eq!(
        !!bitboards::BLACK_START_ROOKS ^ bitboards::BLACK_START_ROOKS,
        BitBoard::empty()
    );

    // iterator
    let mut count = 0;
    for sq in bitboards::BLACK_START_PAWNS.iter() {
        assert_eq!(sq.rank(), Rank::Seven);
        count += 1;
    }
    assert_eq!(count, 8)
}

#[test]
fn bitboard_flip() {
    assert_eq!(
        bitboards::WHITE_START_PAWNS.flip(),
        bitboards::BLACK_START_PAWNS
    );
    assert_eq!(
        bitboards::WHITE_START_ROOKS,
        bitboards::BLACK_START_ROOKS.flip()
    );

    assert_eq!(
        bitboards::WHITE_START_KNIGHTS.flip().flip(),
        bitboards::BLACK_START_KNIGHTS.flip()
    );
    assert_eq!(
        bitboards::WHITE_START_BISHOPS.flip(),
        bitboards::BLACK_START_BISHOPS.flip().flip()
    );

    assert_eq!(
        bitboards::WHITE_START_QUEENS,
        bitboards::BLACK_START_KINGS.flip()
    );
    assert_eq!(
        bitboards::WHITE_START_KINGS.flip(),
        bitboards::BLACK_START_QUEENS
    );
}

#[test]
fn bitboard_mirror_horizontal() {
    assert_eq!(
        bitboards::WHITE_START_PAWNS.mirror_horizontal(),
        bitboards::BLACK_START_PAWNS
    );
    assert_eq!(
        bitboards::WHITE_START_ROOKS,
        bitboards::BLACK_START_ROOKS.mirror_horizontal()
    );

    assert_eq!(
        bitboards::WHITE_START_BISHOPS,
        bitboards::WHITE_START_BISHOPS
            .mirror_horizontal()
            .mirror_horizontal()
    );

    assert_eq!(
        bitboards::WHITE_START_QUEENS,
        bitboards::BLACK_START_QUEENS.mirror_horizontal()
    );
    assert_eq!(
        bitboards::WHITE_START_KINGS.mirror_horizontal(),
        bitboards::BLACK_START_KINGS
    );
    assert_eq!(
        Rank::Five.to_bitboard().mirror_horizontal(),
        Rank::Four.to_bitboard()
    );
    assert_eq!(
        File::A.to_bitboard().mirror_horizontal(),
        File::A.to_bitboard()
    );
}

#[test]
fn bitboard_mirror_diag() {
    assert_eq!(
        Rank::Eight.to_bitboard().mirror_diag(),
        File::H.to_bitboard()
    );
    assert_eq!(
        File::C.to_bitboard().mirror_diag(),
        Rank::Three.to_bitboard()
    );
    assert_eq!(
        File::G.to_bitboard().mirror_diag(),
        Rank::Seven.to_bitboard()
    );
    assert_eq!(
        Rank::Five.to_bitboard().mirror_diag(),
        File::E.to_bitboard()
    );

    assert_eq!(
        Square::from_coordinates(File::B, Rank::Five)
            .to_bitboard()
            .mirror_diag(),
        Square::from_coordinates(File::E, Rank::Two).to_bitboard()
    );
}

#[test]
fn square_from_coordinates() {
    // TODO: Complete set of tests for this -> use those random test things?
    assert_eq!(Square::new(0), Square::from_coordinates(File::A, Rank::One));
    assert_eq!(Square::new(3), Square::from_coordinates(File::D, Rank::One));
    assert_eq!(Square::new(7), Square::from_coordinates(File::H, Rank::One));
    assert_eq!(Square::new(8), Square::from_coordinates(File::A, Rank::Two));
    assert_eq!(
        Square::new(27),
        Square::from_coordinates(File::D, Rank::Four)
    );
    assert_eq!(
        Square::new(47),
        Square::from_coordinates(File::H, Rank::Six)
    );
    assert_eq!(
        Square::new(63),
        Square::from_coordinates(File::H, Rank::Eight)
    );

    let square = Square::from_coordinates(File::C, Rank::Two);
    println!("{}", square.file());
    assert_eq!(square.file(), File::C);
    assert_eq!(square.rank(), Rank::Two);

    // TODO:  Random number test thing
    debug_assert!(catch_unwind(|| Square::new(64)).is_err());
}

#[test]
fn square_flip() {
    assert_eq!(
        Square::from_coordinates(File::A, Rank::One).flip(),
        Square::from_coordinates(File::H, Rank::Eight)
    );
    assert_eq!(
        Square::from_coordinates(File::C, Rank::Three).flip(),
        Square::from_coordinates(File::F, Rank::Six)
    );
    assert_eq!(
        Square::from_coordinates(File::C, Rank::Six).flip(),
        Square::from_coordinates(File::F, Rank::Three)
    );
}

#[test]
fn square_mirror_horizontal() {
    assert_eq!(
        Square::from_coordinates(File::A, Rank::One).mirror_horizontal(),
        Square::from_coordinates(File::A, Rank::Eight)
    );
    assert_eq!(
        Square::from_coordinates(File::C, Rank::Three).mirror_horizontal(),
        Square::from_coordinates(File::C, Rank::Six)
    );
    assert_eq!(
        Square::from_coordinates(File::C, Rank::Six).mirror_horizontal(),
        Square::from_coordinates(File::C, Rank::Three)
    );
}

#[test]
fn square_mirror_diagonal() {
    assert_eq!(
        Square::from_coordinates(File::A, Rank::One).mirror_diag(),
        Square::from_coordinates(File::A, Rank::One)
    );
    assert_eq!(
        Square::from_coordinates(File::C, Rank::Three).mirror_diag(),
        Square::from_coordinates(File::C, Rank::Three)
    );
    assert_eq!(
        Square::from_coordinates(File::C, Rank::Eight).mirror_diag(),
        Square::from_coordinates(File::H, Rank::Three)
    );
    assert_eq!(
        Square::from_coordinates(File::G, Rank::Two).mirror_diag(),
        Square::from_coordinates(File::B, Rank::Seven)
    );
    assert_eq!(
        Square::from_coordinates(File::E, Rank::Four).mirror_diag(),
        Square::from_coordinates(File::D, Rank::Five)
    );
}

#[test]
fn square_diag() {
    assert_eq!(
        Square::from_coordinates(File::A, Rank::One).diagonal(),
        BitBoard::new(0x8040201008040201)
    );
    assert_eq!(
        Square::from_coordinates(File::A, Rank::Two).diagonal(),
        BitBoard::new(0x4020100804020100)
    );
    assert_eq!(
        Square::from_coordinates(File::B, Rank::One).diagonal(),
        BitBoard::new(0x80402010080402)
    );
    assert_eq!(
        Square::from_coordinates(File::H, Rank::Three).diagonal(),
        BitBoard::new(0x804020)
    );
    assert_eq!(
        Square::from_coordinates(File::H, Rank::Six).diagonal(),
        BitBoard::new(0x804020100804)
    );
    assert_eq!(
        Square::from_coordinates(File::F, Rank::Three).diagonal(),
        BitBoard::new(0x8040201008)
    );
    assert_eq!(
        Square::from_coordinates(File::F, Rank::Eight).diagonal(),
        BitBoard::new(0x2010080402010000)
    );
    assert_eq!(
        Square::from_coordinates(File::H, Rank::One).diagonal(),
        BitBoard::new(0x80)
    );
}

#[test]
fn square_antidiag() {
    assert_eq!(
        Square::from_coordinates(File::A, Rank::One).antidiagonal(),
        BitBoard::new(0x1)
    );
    assert_eq!(
        Square::from_coordinates(File::A, Rank::Two).antidiagonal(),
        BitBoard::new(0x102)
    );
    assert_eq!(
        Square::from_coordinates(File::B, Rank::One).antidiagonal(),
        BitBoard::new(0x102)
    );
    assert_eq!(
        Square::from_coordinates(File::H, Rank::Three).antidiagonal(),
        BitBoard::new(0x408102040800000)
    );
    assert_eq!(
        Square::from_coordinates(File::H, Rank::Six).antidiagonal(),
        BitBoard::new(0x2040800000000000)
    );
    assert_eq!(
        Square::from_coordinates(File::F, Rank::Three).antidiagonal(),
        BitBoard::new(0x102040810204080)
    );
    assert_eq!(
        Square::from_coordinates(File::F, Rank::Eight).antidiagonal(),
        BitBoard::new(0x2040800000000000)
    );
}

#[test]
fn rank() {
    //TODO: Complete set of tests for this.
    assert_eq!(Rank::Two.to_index(), 1);
    assert_eq!(Rank::from_index(6), Rank::Seven);

    assert_eq!(Rank::One.to_bitboard(), BitBoard(0xFF));
    assert_eq!(Rank::Two.to_bitboard(), BitBoard(0xFF00));
    assert_eq!(Rank::Three.to_bitboard(), BitBoard(0xFF0000));
    assert_eq!(Rank::Four.to_bitboard(), BitBoard(0xFF000000));
    assert_eq!(Rank::Five.to_bitboard(), BitBoard(0xFF00000000));
    assert_eq!(Rank::Six.to_bitboard(), BitBoard(0xFF0000000000));
    assert_eq!(Rank::Seven.to_bitboard(), BitBoard(0xFF000000000000));
    assert_eq!(Rank::Eight.to_bitboard(), BitBoard(0xFF00000000000000));
}

#[test]
fn file() {
    // TODO Complete set of tests for these.
    assert_eq!(File::B.to_index(), 1);
    assert_eq!(File::from_index(6), File::G);

    assert_eq!(File::A.to_bitboard(), BitBoard(0x101010101010101));
    assert_eq!(File::D.to_bitboard(), BitBoard(0x808080808080808));
    assert_eq!(File::H.to_bitboard(), BitBoard(0x8080808080808080));
}

#[test]
fn game_state() {
    let state = GameState::start_position(); // TODO, test the board.

    println!("{}", state);
}
