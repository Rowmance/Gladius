use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use std::fmt::{Debug, Formatter, Result};
use bit_reverse::ParallelReverse;

/// Represents a 64-bit bitboard.
#[derive(PartialEq, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn flip(&self) -> Self {
        BitBoard(self.0.swap_bits())
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
// Debug
impl Debug for BitBoard {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\n{}:\n+-+-+-+-+-+-+-+-+-+\n", self.0);
        for rank in 0..8 {
            write!(f, "| ");
            for file in 0..8 {
                write!(f, "{} ", if self.0 & (1 << ((7 - rank) * 8 + file)) > 0 { "X" } else { "-" });
            }
            write!(f, "|\n");
        }
        write!(f, "+-+-+-+-+-+-+-+-+-+")
    }
}
