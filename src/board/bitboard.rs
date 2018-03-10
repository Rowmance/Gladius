//! A 64-bit bitboard.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub, SubAssign};
use std::fmt::{Display, Formatter, Result};
use std::u64;
use std::num::Wrapping;

use bit_reverse::ParallelReverse;

use board::square::Square;

/// Represents a 64-bit bitboard.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Creates a new instance from the given 64-bit integer.
    pub fn new(val: u64) -> Self {
        BitBoard(val)
    }

    /// Flips the bitboard such that it's from the perspective of the other player.
    pub fn flip(&self) -> Self {
        BitBoard(self.0.swap_bits())
    }

    /// Mirrors the board horizontally.
    pub fn mirror_horizontal(&self) -> Self {
        BitBoard(self.0.swap_bytes())
    }

    /// Mirrors the board along the A1-H8 diagonal
    pub fn mirror_diag(&self) -> Self {
        static K1: BitBoard = BitBoard(0x5500550055005500);
        static K2: BitBoard = BitBoard(0x3333000033330000);
        static K4: BitBoard = BitBoard(0x0f0f0f0f00000000);
        let mut t = K4 & (*self ^ (*self << 28));
        let mut x = *self ^ (t ^ (t >> 28));
        t = K2 & (x ^ (x << 14));
        x ^= t ^ (t >> 14);
        t = K1 & (x ^ (x << 7));
        x ^= t ^ (t >> 7);
        x
    }

    /// Counts the number of set bits on the bitboard.
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns an iterable bitboard.
    pub fn iter(&self) -> BitBoardIter {
        BitBoardIter(self.0)
    }

    /// Returns true if the given square is set in the bitboard.
    pub fn is_square_set(&self, square: Square) -> bool {
        (1 << square.to_index()) & self.0 > 0
    }

    /// Returns a copy of the bitboard with the given square set
    pub fn set_square(&self, square: Square) -> Self {
        BitBoard(self.0 | square.to_bitboard().0)
    }

    /// Returns a copy of the bitboard with the given square unset
    pub fn unset_square(&self, square: Square) -> Self {
        BitBoard(self.0 & !square.to_bitboard().0)
    }

    /// Returns true if the bitboard is empty.
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns a copy of the bitboard with the given square toggled
    pub fn toggle_square(&self, square: Square) -> Self {
        BitBoard(self.0 ^ square.to_bitboard().0)
    }

    /// Return an empty bitboard.
    pub fn empty() -> Self {
        static EMPTY: BitBoard = BitBoard(0);
        EMPTY
    }

    /// Return a full bitboard.
    pub fn full() -> Self {
        static FULL: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);
        FULL
    }

    /// Returns the underlying u64.
    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

// ---------------------------------------------------------------------------
// Operations
impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = BitBoard(self.0 ^ rhs.0)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

impl Shl<usize> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<usize> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self {
        BitBoard(self.0 >> rhs)
    }
}

impl Sub for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        BitBoard((Wrapping(self.0) - Wrapping(rhs.0)).0)
    }
}

impl SubAssign for BitBoard {
    fn sub_assign(&mut self, rhs: Self) {
        *self = BitBoard((Wrapping(self.0) - Wrapping(rhs.0)).0)
    }
}

// ---------------------------------------------------------------------------
// Display
impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = String::new();
        str.push_str(format!("{:#X}", self.0).as_str());
        str.push_str("\n+-+-+-+-+-+-+-+-+-+\n");
        for rank in 0..8 {
            str.push_str("| ");
            for file in 0..8 {
                let char = if self.0 & (1 << ((7 - rank) * 8 + file)) > 0 {
                    "X "
                } else {
                    "- "
                };
                str.push_str(char);
            }
            str.push_str("|\n");
        }
        str.push_str("+-+-+-+-+-+-+-+-+-+");
        write!(f, "{}", str)
    }
}

// ---------------------------------------------------------------------------
// Iterator

/// Iterator for a [BitBoard].
#[derive(Debug)]
pub struct BitBoardIter(u64);

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let result = Square::new(self.0.trailing_zeros() as u8);
            self.0 ^= result.to_bitboard().0;
            Some(result)
        }
    }
}
