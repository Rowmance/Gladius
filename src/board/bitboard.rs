use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use std::fmt::{Formatter, Result, Display};
use bit_reverse::ParallelReverse;

use board::square::Square;
use std::u64;

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

    /// Counts the number of set bits on the bitboard.
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    // Returns an iterable BitBoard
    pub fn iter(&self) -> BitBoardIter {
        BitBoardIter(self.0)
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

// ---------------------------------------------------------------------------
// Display
impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = String::new();
        str.push_str(self.0.to_string().as_str());
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
