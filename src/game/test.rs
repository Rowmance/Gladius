//! Tests for the game module.

use board::bitboard::BitBoard;
use board::square::Square;
use board::rank::Rank;
use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::board::Board;
use game::valid_moves;

#[test]
fn valid_moves_rook() {
    macro_rules! test_rook {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            assert_eq!(valid_moves::rook(square), BitBoard::new($expected));
        )
    }

    test_rook!(File::A, Rank::One, 0x1010101010101FE);
    test_rook!(File::B, Rank::Two, 0x20202020202FD02);
    test_rook!(File::C, Rank::Three, 0x404040404FB0404);
    test_rook!(File::D, Rank::Four, 0x8080808F7080808);
    test_rook!(File::E, Rank::Five, 0x101010EF10101010);
    test_rook!(File::F, Rank::Six, 0x2020DF2020202020);
    test_rook!(File::G, Rank::Seven, 0x40BF404040404040);
    test_rook!(File::H, Rank::Eight, 0x7F80808080808080);

    test_rook!(File::H, Rank::One, 0x808080808080807F);
    test_rook!(File::G, Rank::Two, 0x404040404040BF40);
    test_rook!(File::F, Rank::Three, 0x2020202020DF2020);
    test_rook!(File::E, Rank::Four, 0x10101010EF101010);
    test_rook!(File::D, Rank::Five, 0x80808F708080808);
    test_rook!(File::C, Rank::Six, 0x404FB0404040404);
    test_rook!(File::B, Rank::Seven, 0x2FD020202020202);
    test_rook!(File::A, Rank::Eight, 0xFE01010101010101);
}

#[test]
fn valid_moves_bishop() {
    macro_rules! test_bishop {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            assert_eq!(valid_moves::bishop(square), BitBoard::new($expected));
        )
    }

    test_bishop!(File::A, Rank::Four, 0x1008040200020408);
    test_bishop!(File::B, Rank::Four, 0x2010080500050810);
    test_bishop!(File::C, Rank::Four, 0x4020110A000A1120);
    test_bishop!(File::D, Rank::Four, 0x8041221400142241);
    test_bishop!(File::E, Rank::Four, 0x182442800284482);
    test_bishop!(File::F, Rank::Four, 0x204885000508804);
    test_bishop!(File::G, Rank::Four, 0x40810A000A01008);
    test_bishop!(File::H, Rank::Four, 0x810204000402010);

    test_bishop!(File::E, Rank::One, 0x182442800);
    test_bishop!(File::E, Rank::Two, 0x18244280028);
    test_bishop!(File::E, Rank::Three, 0x1824428002844);
    test_bishop!(File::E, Rank::Four, 0x182442800284482);
    test_bishop!(File::E, Rank::Five, 0x8244280028448201);
    test_bishop!(File::E, Rank::Six, 0x4428002844820100);
    test_bishop!(File::E, Rank::Seven, 0x2800284482010000);
    test_bishop!(File::E, Rank::Eight, 0x28448201000000);

    test_bishop!(File::H, Rank::One, 0x102040810204000);
    test_bishop!(File::H, Rank::Eight, 0x40201008040201);
    test_bishop!(File::A, Rank::One, 0x8040201008040200);
    test_bishop!(File::A, Rank::Eight, 0x2040810204080);
}