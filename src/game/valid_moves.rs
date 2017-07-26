//! The movement of chess pieces

use board::square::Square;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::file::File;
use board::bitboard::BitBoard;

// TODO: Can all of these be generated at build time? There are only (up to) 64 possibilities per piece.

/// Gets the valid pawn moves for a given square.
pub fn pawn_moves(square: Square, player: Player) -> BitBoard {
    let homefile = match player {
        Player::White => File::B,
        Player::Black => File::G
    };
    if square.file() == homefile {
        let forwardbb = match player {
            Player::White => {
                square.file().next().unwrap().to_bitboard()
                    | square.file().next().unwrap().next().unwrap().to_bitboard()
            }
            Player::Black => {
                square.file().prev().unwrap().to_bitboard()
                    | square.file().prev().unwrap().prev().unwrap().to_bitboard()
            }
        };
        return forwardbb & square.rank().to_bitboard();
    }
    let forward = match player {
        Player::White => square.file().next(),
        Player::Black => square.file().prev()
    };
    forward.map_or(BitBoard::empty(), |file| file.to_bitboard() & square.rank().to_bitboard())
}

/// Gets the valid rook moves for a given square.
pub fn rook(square: Square) -> BitBoard {
    square.rank().to_bitboard() ^ square.file().to_bitboard()
}

/// Gets the valid bishop moves for a given square.
pub fn bishop(square: Square) -> BitBoard {

    // Shift the main diagonal as appropriate and use a mask to remove any overflowing squares.
    let main_diff = square.file().to_index() as isize - square.rank().to_index() as isize;
    let main_mask = !BitBoard::new((2 as u64).pow((8 * main_diff.abs()) as u32) - 1);
    let main_dia = if main_diff > 0 {
        BitBoard::new(0x8040201008040201 << main_diff) & main_mask.flip()
    } else {
        BitBoard::new(0x8040201008040201 >> main_diff.abs()) & main_mask
    };

    let anti_diff = ( -7 + square.rank().to_index() as isize) + square.file().to_index() as isize;

    let anti_dia = if anti_diff >= 0 {
        let y: u32 = (8 * (anti_diff.abs())) as u32;
        let foo2 = !((2 as u64).pow(y) - 1);
        BitBoard::new((0x0102040810204080 & foo2) << anti_diff)
    } else {
        let y: u32 = (8 * (8 - anti_diff.abs())) as u32;
        let foo2 = 0xFFFFFFFFFFFFFFFF ^ ((2 as u64).pow(y) - 1);
        BitBoard::new((0x0102040810204080 & !foo2) >> anti_diff.abs())
    };

    main_dia ^ anti_dia
}