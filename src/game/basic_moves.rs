//! The movement of chess pieces on an empty board.

use board::square::Square;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::file::File;
use board::bitboard::BitBoard;

// TODO: Can all of these be generated at build time? There are only (up to) 64 possibilities per piece.
// TODO: If not, cache these.

/// Returns the valid pawn moves for a given square and player.
pub fn pawn_moves(square: Square, player: Player) -> BitBoard {
    let homerank = match player {
        Player::White => Rank::Two,
        Player::Black => Rank::Seven
    };
    if square.rank() == homerank {
        let forwardbb = match player {
            Player::White => {
                square.rank().next().unwrap().to_bitboard()
                    | square.rank().next().unwrap().next().unwrap().to_bitboard()
            }
            Player::Black => {
                square.rank().prev().unwrap().to_bitboard()
                    | square.rank().prev().unwrap().prev().unwrap().to_bitboard()
            }
        };
        return forwardbb & square.file().to_bitboard();
    }
    let forward = match player {
        Player::White => square.rank().next(),
        Player::Black => square.rank().prev()
    };

    forward.map_or(BitBoard::empty(), |rank| rank.to_bitboard() & square.file().to_bitboard())
}

/// Returns the valid pawn attacks for a given square and player.
pub fn pawn_attacks(square: Square, player: Player) -> BitBoard {
    let sides = square.file().prev().map_or(BitBoard::empty(), |file| file.to_bitboard())
        | square.file().next().map_or(BitBoard::empty(), |file| file.to_bitboard());

    let forward = match player {
        Player::White => square.rank().next(),
        Player::Black => square.rank().prev()
    }.map_or(BitBoard::empty(), |rank| rank.to_bitboard());

    forward & sides
}

/// Returns the valid rook moves for a given square.
pub fn rook(square: Square) -> BitBoard {
    square.rank().to_bitboard() ^ square.file().to_bitboard()
}

/// Returns the valid bishop moves for a given square.
pub fn bishop(square: Square) -> BitBoard {

    // Shift the main diagonal as appropriate and use a mask to remove any overflowing squares.
    let main_diff = square.file().to_index() as isize - square.rank().to_index() as isize;
    let main_mask = !BitBoard::new((2 as u64).pow((8 * main_diff.abs()) as u32) - 1);
    let main_dia = if main_diff > 0 {
        BitBoard::new(0x8040201008040201 << main_diff) & main_mask.flip()
    } else {
        BitBoard::new(0x8040201008040201 >> main_diff.abs()) & main_mask
    };

    // Anti-diagonal needs to be masked and then shifted. // TODO: does it?
    let anti_diff = ( -7 + square.rank().to_index() as isize) + square.file().to_index() as isize;
    let anti_dia = if anti_diff >= 0 {
        let anti_mask = !((2 as u64).pow(8 * (anti_diff.abs()) as u32) - 1);
        BitBoard::new((0x0102040810204080 & anti_mask) << anti_diff)
    } else {
        let anti_mask = 0xFFFFFFFFFFFFFFFF ^ ((2 as u64).pow(8 * (8 - anti_diff.abs()) as u32) - 1);
        BitBoard::new((0x0102040810204080 & !anti_mask) >> anti_diff.abs())
    };

    main_dia ^ anti_dia
}

/// Returns the queen moves for a given square.
pub fn queen(square: Square) -> BitBoard {
    return rook(square) | bishop(square);
}

/// Returns the knight moves for a given square.
pub fn knight(square: Square) -> BitBoard {
    // TODO: these can be consts
    let not_a = !File::A.to_bitboard().to_u64();
    let not_ab = !((File::A.to_bitboard() | File::B.to_bitboard()).to_u64());
    let not_h = !File::H.to_bitboard().to_u64();
    let not_gh = !((File::H.to_bitboard() | File::G.to_bitboard()).to_u64());

    // Shift squares without overflowing
    let index = square.to_bitboard().to_u64();
    let mut result: u64 = 0;
    result |= index << 17 & not_a;
    result |= index << 10 & not_ab;
    result |= index >> 6 & not_ab;
    result |= index >> 15 & not_a;
    result |= index >> 17 & not_h;
    result |= index >> 10 & not_gh;
    result |= index << 6 & not_gh;
    result |= index << 15 & not_h;
    BitBoard::new(result)
}

/// Returns the king moves for a given square.
pub fn king(square: Square) -> BitBoard {
    let not_a = !File::A.to_bitboard().to_u64();
    let not_h = !File::H.to_bitboard().to_u64();
    
    let mut result: u64 = 0;
    let index = square.to_bitboard().to_u64();
    result |= index << 1 & not_a;
    result |= index << 7 & not_h;
    result |= index << 8;
    result |= index << 9 & not_a;
    result |= index >> 1 & not_h;
    result |= index >> 7 & not_a;
    result |= index >> 8;
    result |= index >> 9 & not_h;
    BitBoard::new(result)
}