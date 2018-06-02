use board::bitboard::BitBoard;
use board::file::File;
use board::rank::Rank;
use board::square::Square;
use std::panic::catch_unwind;

#[test]
fn square_from_coordinates() {
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
