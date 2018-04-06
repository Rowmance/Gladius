use board::rank::Rank;
use board::bitboard::BitBoard;

#[test]
fn rank() {
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
