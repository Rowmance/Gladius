//! A collection of commonly used bitboards.

// Don't trigger dead code lint if these aren't used.
#![allow(dead_code)]

use board::bitboard::BitBoard;

/// An empty bitboard.
pub const EMPTY: BitBoard = BitBoard(0);

//---------------------------------------------------------------------------
// White Player

/// White player starting pawn positions.
pub const WHITE_START_PAWNS: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000);

/// White player starting rook positions.
pub const WHITE_START_ROOKS: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001);

/// White player starting knight positions.
pub const WHITE_START_KNIGHTS: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010);

/// White player starting bishop positions.
pub const WHITE_START_BISHOPS: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100);

/// White player starting queen position.
pub const WHITE_START_QUEENS: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000);

/// White player starting king position.
pub const WHITE_START_KINGS: BitBoard =
    BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000);

//---------------------------------------------------------------------------
// Black Player

/// Black player starting pawn positions.
pub const BLACK_START_PAWNS: BitBoard =
    BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000);

/// Black player starting rook positions.
pub const BLACK_START_ROOKS: BitBoard =
    BitBoard(0b1000000100000000000000000000000000000000000000000000000000000000);

/// Black player starting knight positions.
pub const BLACK_START_KNIGHTS: BitBoard =
    BitBoard(0b0100001000000000000000000000000000000000000000000000000000000000);

/// Black player starting bishop positions.
pub const BLACK_START_BISHOPS: BitBoard =
    BitBoard(0b0010010000000000000000000000000000000000000000000000000000000000);

/// Black player starting queen position.
pub const BLACK_START_QUEENS: BitBoard =
    BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000);

/// Black player starting king position.
pub const BLACK_START_KINGS: BitBoard =
    BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000);
