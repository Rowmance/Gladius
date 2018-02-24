//! Tests for the game module.

use board::bitboard::BitBoard;
use board::square::Square;
use board::rank::Rank;
use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::board::Board;
use game::basic_moves;
use game::semilegal_moves;

#[test]
fn basic_moves_rook() {
    macro_rules! test_rook {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            assert_eq!(basic_moves::rook(square), BitBoard::new($expected));
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
fn basic_moves_bishop() {
    macro_rules! test_bishop {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            assert_eq!(basic_moves::bishop(square), BitBoard::new($expected));
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

#[test]
fn basic_moves_queen() {
    macro_rules! test_queen {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            assert_eq!(basic_moves::queen(square), BitBoard::new($expected));
        )
    }

    test_queen!(File::C, Rank::Three, 0x844424150EFB0E15);
    test_queen!(File::F, Rank::Six, 0xA870DF70A8242221);
    test_queen!(File::C, Rank::Six, 0x150EFB0E15244484);
    test_queen!(File::F, Rank::Three, 0x212224A870DF70A8);
}

#[test]
fn basic_moves_king() {
    macro_rules! test_king {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = basic_moves::king(square);
            println!("\n{} king moves: {}", square, moves);
            assert_eq!(moves, BitBoard::new($expected));
        )
    }

    test_king!(File::C, Rank::Three, 0xE0A0E00);
    test_king!(File::F, Rank::Six, 0x70507000000000);
    test_king!(File::C, Rank::Six, 0xE0A0E00000000);
    test_king!(File::F, Rank::Three, 0x70507000);
    test_king!(File::A, Rank::One, 0x302);
    test_king!(File::A, Rank::Eight, 0x203000000000000);
    test_king!(File::H, Rank::One, 0xC040);
    test_king!(File::H, Rank::Eight, 0x40C0000000000000);
}

#[test]
fn basic_moves_pawn() {
    macro_rules! test_pawn {
        ($file:expr, $rank:expr, $player:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            assert_eq!(basic_moves::pawn_moves(square, $player), BitBoard::new($expected));
        )
    }

    test_pawn!(File::C, Rank::Four, Player::White, 0x400000000);
    test_pawn!(File::C, Rank::Two, Player::White, 0x4040000);
    test_pawn!(File::C, Rank::Seven, Player::White, 0x400000000000000);
    test_pawn!(File::C, Rank::One, Player::White, 0x400);
    test_pawn!(File::C, Rank::Eight, Player::White, 0);

    test_pawn!(File::C, Rank::Four, Player::Black, 0x40000);
    test_pawn!(File::C, Rank::Two, Player::Black, 0x4);
    test_pawn!(File::C, Rank::Seven, Player::Black, 0x40400000000);
    test_pawn!(File::C, Rank::One, Player::Black, 0);
    test_pawn!(File::C, Rank::Eight, Player::Black, 0x4000000000000);
}

#[test]
fn basic_attacks_pawn() {
    macro_rules! test_pawn {
        ($file:expr, $rank:expr, $player:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = basic_moves::pawn_attacks(square, $player);
            println!("\n{} {} pawn attacks: {}", square, $player, moves);
            assert_eq!(moves, BitBoard::new($expected));
        )
    }

    test_pawn!(File::C, Rank::Four, Player::White, 0xA00000000);
    test_pawn!(File::C, Rank::Two, Player::White, 0xA0000);
    test_pawn!(File::C, Rank::Seven, Player::White, 0xA00000000000000);
    test_pawn!(File::C, Rank::One, Player::White, 0xA00);
    test_pawn!(File::C, Rank::Eight, Player::White, 0);

    test_pawn!(File::C, Rank::Four, Player::Black, 0xA0000);
    test_pawn!(File::C, Rank::Two, Player::Black, 0xA);
    test_pawn!(File::C, Rank::Seven, Player::Black, 0xA0000000000);
    test_pawn!(File::C, Rank::One, Player::Black, 0);
    test_pawn!(File::C, Rank::Eight, Player::Black, 0xA000000000000);
}

#[test]
fn basic_moves_knight() {
    macro_rules! test_knight {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = basic_moves::knight(square);
            println!("\n{} knight attacks: {}", square, moves);
            assert_eq!(moves, BitBoard::new($expected));
        )
    }

    test_knight!(File::C, Rank::Four, 0xA1100110A00);

    test_knight!(File::C, Rank::Seven, 0x1100110A00000000);
    test_knight!(File::C, Rank::Eight, 0x110A0000000000);
    test_knight!(File::C, Rank::One, 0xA1100);
    test_knight!(File::C, Rank::Two, 0xA110011);

    test_knight!(File::A, Rank::Five, 0x2040004020000);
    test_knight!(File::B, Rank::Five, 0x5080008050000);
    test_knight!(File::G, Rank::Five, 0xA0100010A00000);
    test_knight!(File::H, Rank::Five, 0x40200020400000);

    test_knight!(File::H, Rank::One, 0x402000);
    test_knight!(File::H, Rank::Eight, 0x20400000000000);
    test_knight!(File::A, Rank::One, 0x20400);
    test_knight!(File::A, Rank::Eight, 0x4020000000000);
}

// -----------------------------------
#[test]
fn semilegal_moves_pawn() {
    macro_rules! test_pawn {
        ($own_file:expr, $own_rank:expr, $opp_file:expr, $opp_rank:expr, $player:expr, $expected:expr) => (
            let own_square = Square::from_coordinates($own_file, $own_rank);
            let blockers = Square::from_coordinates($opp_file, $opp_rank).to_bitboard();
            let moves = semilegal_moves::pawn_moves(own_square, $player, blockers);
            
            println!("\n{} {} pawn moves blocked by:\n{}\nGives: {}", own_square, $player, blockers, moves);
            assert_eq!(moves, BitBoard::new($expected));
        )
    }

    test_pawn!(File::A, Rank::Five, File::A, Rank::Six, Player::White, 0x0);
    test_pawn!(File::A, Rank::Five, File::A, Rank::Seven, Player::White, 0x10000000000);
    test_pawn!(File::A, Rank::Five, File::A, Rank::Eight, Player::White, 0x10000000000);

    test_pawn!(File::B, Rank::Two, File::B, Rank::Three, Player::White, 0x0);
    test_pawn!(File::B, Rank::Two, File::B, Rank::Four, Player::White, 0x20000);
    test_pawn!(File::B, Rank::Two, File::B, Rank::Five, Player::White, 0x2020000);

    test_pawn!(File::D, Rank::Five, File::D, Rank::Four, Player::Black, 0x0);
    test_pawn!(File::D, Rank::Five, File::D, Rank::Three, Player::Black, 0x8000000);
    test_pawn!(File::D, Rank::Five, File::D, Rank::Two, Player::Black, 0x8000000);

    test_pawn!(File::H, Rank::Seven, File::H, Rank::Six, Player::Black, 0x0);
    test_pawn!(File::H, Rank::Seven, File::H, Rank::Five, Player::Black, 0x800000000000);
    test_pawn!(File::H, Rank::Seven, File::H, Rank::Four, Player::Black, 0x808000000000);
}

#[test]
fn semilegal_attacks_pawn() {
    macro_rules! test_pawn {
        ($file:expr, $rank:expr, $targets:expr, $player:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let targets = semilegal_moves::pawn_attacks(square, $player, $targets);
            
            println!("\n{} {} pawn attacks against:\n{}\nGives: {}", square, $player, $targets, targets);
            assert_eq!(targets, BitBoard::new($expected));
        )
    }

    let targets = Rank::Five.to_bitboard();
    test_pawn!(File::A, Rank::Four, targets, Player::White, 0x200000000);
    test_pawn!(File::H, Rank::Four, targets, Player::White, 0x4000000000);
    test_pawn!(File::B, Rank::Four, targets, Player::White, 0x500000000);
    test_pawn!(File::B, Rank::Six, targets, Player::White, 0x0);

    test_pawn!(File::A, Rank::Six, targets, Player::Black, 0x200000000);
    test_pawn!(File::H, Rank::Six, targets, Player::Black, 0x4000000000);
    test_pawn!(File::B, Rank::Six, targets, Player::Black, 0x500000000);
    test_pawn!(File::B, Rank::Four, targets, Player::Black, 0x0);
}

#[test]
fn semilegal_moves_knight() {
    // A board with a varied set of squares set:
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - - |
    // | - - - - - - X - |
    // | - - - X X - - - |
    // | - X - - - - - - |
    // | - - - - - - - - |
    // | - X - X - X - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // +-+-+-+-+-+-+-+-+-+
    let board = BitBoard::empty()
        .set_square(Square::from_coordinates(File::B, Rank::Three))
        .set_square(Square::from_coordinates(File::B, Rank::Five))
        .set_square(Square::from_coordinates(File::D, Rank::Three))
        .set_square(Square::from_coordinates(File::D, Rank::Six))
        .set_square(Square::from_coordinates(File::E, Rank::Six))
        .set_square(Square::from_coordinates(File::F, Rank::Three))
        .set_square(Square::from_coordinates(File::G, Rank::Seven));

    macro_rules! test_knight {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::knight_moves(square, board);
            
            println!("\n{} knight moves against:\n{}\nGives: {}", square, board, moves);
            assert_eq!(moves, BitBoard::new($expected));
        );
    }

    test_knight!(File::D, Rank::Four, 0x42000001400);
    test_knight!(File::C, Rank::One, 0x1100);
    test_knight!(File::E, Rank::Five, 0x28440044000000);
    test_knight!(File::H, Rank::Eight, 0x20400000000000);
}

#[test]
fn semilegal_attacks_knight() {
    // A board with a varied set of squares set:
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - - |
    // | - - - - - - X - |
    // | - - - X X - - - |
    // | - X - - - - - - |
    // | - - - - - - - - |
    // | - X - X - X - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // +-+-+-+-+-+-+-+-+-+
    let board = BitBoard::empty()
        .set_square(Square::from_coordinates(File::B, Rank::Three))
        .set_square(Square::from_coordinates(File::B, Rank::Five))
        .set_square(Square::from_coordinates(File::D, Rank::Three))
        .set_square(Square::from_coordinates(File::D, Rank::Six))
        .set_square(Square::from_coordinates(File::E, Rank::Six))
        .set_square(Square::from_coordinates(File::F, Rank::Three))
        .set_square(Square::from_coordinates(File::G, Rank::Seven));

    macro_rules! test_knight {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::knight_attacks(square, board);
            
            println!("\n{} knight attacks against:\n{}\nGives: {}", square, board, moves);
            assert_eq!(moves, BitBoard::new($expected));
        );
    }

    test_knight!(File::D, Rank::Four, 0x100200220000);
    test_knight!(File::C, Rank::One, 0xA0000);
    test_knight!(File::E, Rank::Five, 0x280000);
    test_knight!(File::H, Rank::Eight, 0x0);
}

#[test]
fn semilegal_moves_rook() {
    // A board with a varied set of squares set:
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - - |
    // | - - - - - - X - |
    // | - - - X X - - - |
    // | - X - - - - - - |
    // | - - - - - - - - |
    // | - X - X - X - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // +-+-+-+-+-+-+-+-+-+
    let board = BitBoard::empty()
        .set_square(Square::from_coordinates(File::B, Rank::Three))
        .set_square(Square::from_coordinates(File::B, Rank::Five))
        .set_square(Square::from_coordinates(File::D, Rank::Three))
        .set_square(Square::from_coordinates(File::D, Rank::Six))
        .set_square(Square::from_coordinates(File::E, Rank::Six))
        .set_square(Square::from_coordinates(File::F, Rank::Three))
        .set_square(Square::from_coordinates(File::G, Rank::Seven));

    macro_rules! test_rook {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::rook_moves(square, board);
            
            println!("\n{} rook moves against:\n{}\nGives: {}", square, board, moves);
            assert_eq!(moves, BitBoard::new($expected));
        );
    }

    test_rook!(File::D, Rank::Four, 0x8F7000000);
    test_rook!(File::C, Rank::One, 0x4040404040404FB);
    test_rook!(File::E, Rank::Four, 0x10EF101010);
    test_rook!(File::H, Rank::Eight, 0x7F80808080808080);
    test_rook!(File::H, Rank::Four, 0x808080807F808080);
    test_rook!(File::C, Rank::Three, 0x404040404000404);
    test_rook!(File::G, Rank::Three, 0x404040804040);
    test_rook!(File::D, Rank::Five, 0xF408000000);
}

#[test]
fn semilegal_attacks_rook() {
    // A board with a varied set of squares set:
    // +-+-+-+-+-+-+-+-+-+
    // | - - - - - - - - |
    // | - - - - - - X - |
    // | - - - O O - - - |
    // | - X - - - - - - |
    // | - - - - - - - - |
    // | - X - X - 0 - - |
    // | - - - - - - - - |
    // | - - - - - - - - |
    // +-+-+-+-+-+-+-+-+-+
    // O: Own   X: Opponent
    let own_board = BitBoard::empty()
        .set_square(Square::from_coordinates(File::F, Rank::Three))
        .set_square(Square::from_coordinates(File::D, Rank::Six))
        .set_square(Square::from_coordinates(File::E, Rank::Six));

    let opponent_board = BitBoard::empty()
        .set_square(Square::from_coordinates(File::B, Rank::Three))
        .set_square(Square::from_coordinates(File::B, Rank::Five))
        .set_square(Square::from_coordinates(File::D, Rank::Three))
        .set_square(Square::from_coordinates(File::G, Rank::Seven));

    macro_rules! test_rook {
        ($file:expr, $rank:expr, $expected:expr) => (
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::rook_attacks(square, own_board, opponent_board);
            
            println!(
                "\n{} rook attacks against:\n{}\nBlocked by {}\nGives: {}", 
                square,
                opponent_board,
                own_board, 
                moves);
            assert_eq!(moves, BitBoard::new($expected));
        );
    }

    test_rook!(File::D, Rank::Four, 0x80000);
    test_rook!(File::C, Rank::One, 0x0);
    test_rook!(File::E, Rank::Four, 0x0);
    test_rook!(File::H, Rank::Eight, 0x0);
    test_rook!(File::H, Rank::Four, 0x0);
    test_rook!(File::C, Rank::Three, 0xA0000);
    test_rook!(File::G, Rank::Five, 0x40000200000000);
    test_rook!(File::D, Rank::Five, 0x200080000);
}
