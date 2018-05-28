//! Types and implementations for chess board representation.
//!
//! This module allows for the representation of any single
//! playable position in a game of Chess. In particular, the
//! module manages:
//!
//! * the positions of the pieces on the board
//! * the player whose turn it is
//! * which castling opportunities are available
//! * whether en-passant is possible, and where
//! * the number of half-turns since the last capture or
//!   pawn advance (for the 50-move draw rule)
//! * the number of full turns which have occurred.

#![allow(dead_code)]
#![allow(unused_must_use)]

pub mod bitboard;
pub mod file;
pub mod piece;
pub mod player;
pub mod rank;
pub mod square;

#[cfg(test)]
mod test;
