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

mod bitboard;
mod bitboards;
mod square;
mod rank;
mod file;
mod piece;
mod player;
mod board;

#[cfg(test)]
mod test;

