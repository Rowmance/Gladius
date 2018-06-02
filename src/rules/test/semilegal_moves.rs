use board::bitboard::BitBoard;
use board::file::File;
use board::player::Player;
use board::rank::Rank;
use board::square::Square;
use rules::semilegal_moves;

#[test]
fn semilegal_moves_pawn() {
    macro_rules! test_pawn {
        (
            $own_file:expr,
            $own_rank:expr,
            $opp_file:expr,
            $opp_rank:expr,
            $player:expr,
            $expected:expr
        ) => {
            let own_square = Square::from_coordinates($own_file, $own_rank);
            let blockers = Square::from_coordinates($opp_file, $opp_rank).to_bitboard();
            let moves = semilegal_moves::pawn_moves(own_square, $player, blockers);

            println!(
                "\n{} {} pawn moves blocked by:\n{}\nGives: {}",
                own_square, $player, blockers, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
    }

    test_pawn!(File::A, Rank::Five, File::A, Rank::Six, Player::White, 0x0);
    test_pawn!(
        File::A,
        Rank::Five,
        File::A,
        Rank::Seven,
        Player::White,
        0x10000000000
    );
    test_pawn!(
        File::A,
        Rank::Five,
        File::A,
        Rank::Eight,
        Player::White,
        0x10000000000
    );

    test_pawn!(File::B, Rank::Two, File::B, Rank::Three, Player::White, 0x0);
    test_pawn!(
        File::B,
        Rank::Two,
        File::B,
        Rank::Four,
        Player::White,
        0x20000
    );
    test_pawn!(
        File::B,
        Rank::Two,
        File::B,
        Rank::Five,
        Player::White,
        0x2020000
    );

    test_pawn!(File::D, Rank::Five, File::D, Rank::Four, Player::Black, 0x0);
    test_pawn!(
        File::D,
        Rank::Five,
        File::D,
        Rank::Three,
        Player::Black,
        0x8000000
    );
    test_pawn!(
        File::D,
        Rank::Five,
        File::D,
        Rank::Two,
        Player::Black,
        0x8000000
    );

    test_pawn!(File::H, Rank::Seven, File::H, Rank::Six, Player::Black, 0x0);
    test_pawn!(
        File::H,
        Rank::Seven,
        File::H,
        Rank::Five,
        Player::Black,
        0x800000000000
    );
    test_pawn!(
        File::H,
        Rank::Seven,
        File::H,
        Rank::Four,
        Player::Black,
        0x808000000000
    );

    test_pawn!(
        File::H,
        Rank::Two,
        File::A,
        Rank::Four,
        Player::White,
        0x80800000
    );
}

#[test]
fn semilegal_attacks_pawn() {
    macro_rules! test_pawn {
        ($file:expr, $rank:expr, $targets:expr, $player:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let targets = semilegal_moves::pawn_attacks(square, $player, $targets);

            println!(
                "\n{} {} pawn attacks against:\n{}\nGives: {}",
                square, $player, $targets, targets
            );
            assert_eq!(targets, BitBoard::new($expected));
        };
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
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::knight_moves(square, board);

            println!(
                "\n{} knight moves against:\n{}\nGives: {}",
                square, board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
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
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::knight_attacks(square, board);

            println!(
                "\n{} knight attacks against:\n{}\nGives: {}",
                square, board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
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
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::rook_moves(square, board);

            println!(
                "\n{} rook moves against:\n{}\nGives: {}",
                square, board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
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
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::rook_attacks(square, own_board, opponent_board);

            println!(
                "\n{} rook attacks against:\n{}\nBlocked by {}\nGives: {}",
                square, opponent_board, own_board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
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

#[test]
fn semilegal_moves_bishop() {
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

    macro_rules! test_bishop {
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::bishop_moves(square, board);

            println!(
                "\n{} bishop moves against:\n{}\nGives: {}",
                square, board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
    }

    test_bishop!(File::D, Rank::Four, 0x1221400142241);
    test_bishop!(File::C, Rank::One, 0x804020110A00);
    test_bishop!(File::E, Rank::Four, 0x182442800000000);
    test_bishop!(File::H, Rank::Eight, 0x0);
    test_bishop!(File::H, Rank::Four, 0x810204000402010);
    test_bishop!(File::C, Rank::Three, 0x20110A000A11);
    test_bishop!(File::G, Rank::Three, 0x10A000A010);
    test_bishop!(File::D, Rank::Five, 0x102040014000000);
    test_bishop!(File::D, Rank::Seven, 0x1400040000000000);
    test_bishop!(File::F, Rank::Eight, 0x10000000000000);
}

#[test]
fn semilegal_attack_bishop() {
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

    macro_rules! test_bishop {
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::bishop_attacks(square, own_board, opponent_board);

            println!(
                "\n{} bishop attacks against:\n{}\nBlocked by {}\nGives: {}",
                square, opponent_board, own_board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
    }

    test_bishop!(File::D, Rank::Four, 0x40000000000000);
    test_bishop!(File::C, Rank::One, 0x0);
    test_bishop!(File::E, Rank::Four, 0x80000);
    test_bishop!(File::H, Rank::Eight, 0x40000000000000);
    test_bishop!(File::H, Rank::Four, 0x0);
    test_bishop!(File::C, Rank::Three, 0x40000000000000);
    test_bishop!(File::G, Rank::Three, 0x0);
    test_bishop!(File::D, Rank::Five, 0x20000);
    test_bishop!(File::D, Rank::Seven, 0x200000000);
    test_bishop!(File::F, Rank::Eight, 0x40000000000000);
}

#[test]
fn semilegal_moves_queen() {
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

    macro_rules! test_queen {
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::queen_moves(square, board);

            println!(
                "\n{} queen moves against:\n{}\nGives: {}",
                square, board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
    }

    test_queen!(File::D, Rank::Four, 0x1221CF7142241);
    test_queen!(File::C, Rank::One, 0x404844424150EFB);
    test_queen!(File::E, Rank::Four, 0x1824438EF101010);
    test_queen!(File::H, Rank::Eight, 0x7F80808080808080);
    test_queen!(File::H, Rank::Four, 0x8890A0C07FC0A090);
    test_queen!(File::C, Rank::Three, 0x40424150E000E15);
    test_queen!(File::G, Rank::Three, 0x4050E080E050);
    test_queen!(File::D, Rank::Five, 0x10204F41C000000);
    test_queen!(File::D, Rank::Seven, 0x1C37040000000000);
    test_queen!(File::F, Rank::Eight, 0xDF30202020000000);
}

#[test]
fn semilegal_attack_queen() {
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

    macro_rules! test_queen {
        ($file:expr, $rank:expr, $expected:expr) => {
            let square = Square::from_coordinates($file, $rank);
            let moves = semilegal_moves::queen_attacks(square, own_board, opponent_board);

            println!(
                "\n{} queen attacks against:\n{}\nBlocked by {}\nGives: {}",
                square, opponent_board, own_board, moves
            );
            assert_eq!(moves, BitBoard::new($expected));
        };
    }

    test_queen!(File::D, Rank::Four, 0x40000000080000);
    test_queen!(File::C, Rank::One, 0x0);
    test_queen!(File::E, Rank::Four, 0x80000);
    test_queen!(File::H, Rank::Eight, 0x40000000000000);
    test_queen!(File::H, Rank::Four, 0x0);
    test_queen!(File::C, Rank::Three, 0x400000000A0000);
    test_queen!(File::G, Rank::Three, 0x40000000000000);
    test_queen!(File::D, Rank::Five, 0x2000A0000);
    test_queen!(File::D, Rank::Seven, 0x40000200000000);
    test_queen!(File::F, Rank::Eight, 0x40000000000000);
}
