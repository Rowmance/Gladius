use board::file::File;
use board::bitboard::BitBoard;

#[test]
fn file() {
    assert_eq!(File::B.to_index(), 1);
    assert_eq!(File::from_index(6), File::G);

    assert_eq!(File::A.to_bitboard(), BitBoard(0x101010101010101));
    assert_eq!(File::D.to_bitboard(), BitBoard(0x808080808080808));
    assert_eq!(File::H.to_bitboard(), BitBoard(0x8080808080808080));
}
