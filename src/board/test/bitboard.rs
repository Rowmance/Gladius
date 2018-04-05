use board::rank::Rank;
use board::file::File;
use board::square::Square;
use board::bitboard::BitBoard;
use board::bitboard;

#[test]
fn bitboard_ops() {
    // xor, and, or operators
    let all_white = bitboard::WHITE_START_PAWNS | bitboard::WHITE_START_ROOKS | bitboard::WHITE_START_KNIGHTS
        | bitboard::WHITE_START_BISHOPS | bitboard::WHITE_START_QUEENS
        | bitboard::WHITE_START_KINGS;

    let white_pieces = all_white ^ bitboard::WHITE_START_PAWNS;
    assert_eq!(
        white_pieces & bitboard::WHITE_START_PAWNS,
        BitBoard::empty()
    );
    assert_eq!(white_pieces & all_white, white_pieces);

    let black_pieces = all_white.flip() ^ bitboard::BLACK_START_PAWNS;
    assert_eq!(black_pieces.flip(), white_pieces);

    let all_black = black_pieces | bitboard::BLACK_START_PAWNS;
    assert_eq!(all_white.flip(), all_black);

    // not operator
    assert_eq!(
        !!bitboard::BLACK_START_ROOKS ^ bitboard::BLACK_START_ROOKS,
        BitBoard::empty()
    );

    // iterator
    let mut count = 0;
    for sq in bitboard::BLACK_START_PAWNS.iter() {
        assert_eq!(sq.rank(), Rank::Seven);
        count += 1;
    }
    assert_eq!(count, 8)
}

#[test]
fn bitboard_flip() {
    assert_eq!(
        bitboard::WHITE_START_PAWNS.flip(),
        bitboard::BLACK_START_PAWNS
    );
    assert_eq!(
        bitboard::WHITE_START_ROOKS,
        bitboard::BLACK_START_ROOKS.flip()
    );

    assert_eq!(
        bitboard::WHITE_START_KNIGHTS.flip().flip(),
        bitboard::BLACK_START_KNIGHTS.flip()
    );
    assert_eq!(
        bitboard::WHITE_START_BISHOPS.flip(),
        bitboard::BLACK_START_BISHOPS.flip().flip()
    );

    assert_eq!(
        bitboard::WHITE_START_QUEENS,
        bitboard::BLACK_START_KINGS.flip()
    );
    assert_eq!(
        bitboard::WHITE_START_KINGS.flip(),
        bitboard::BLACK_START_QUEENS
    );
}

#[test]
fn bitboard_mirror_horizontal() {
    assert_eq!(
        bitboard::WHITE_START_PAWNS.mirror_horizontal(),
        bitboard::BLACK_START_PAWNS
    );
    assert_eq!(
        bitboard::WHITE_START_ROOKS,
        bitboard::BLACK_START_ROOKS.mirror_horizontal()
    );

    assert_eq!(
        bitboard::WHITE_START_BISHOPS,
        bitboard::WHITE_START_BISHOPS
            .mirror_horizontal()
            .mirror_horizontal()
    );

    assert_eq!(
        bitboard::WHITE_START_QUEENS,
        bitboard::BLACK_START_QUEENS.mirror_horizontal()
    );
    assert_eq!(
        bitboard::WHITE_START_KINGS.mirror_horizontal(),
        bitboard::BLACK_START_KINGS
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
fn bitboard_to_square() {
    let square = Square::from_coordinates(File::F, Rank::Three);
    assert_eq!(square, square.to_bitboard().to_square());
}
