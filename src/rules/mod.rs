//! Chess rules module.
//!
//! This module contains rules around the movement of pieces and
//! general victory/defeat conditions.
//!
//! This includes:
//! * Which pieces can move where
//! * Castling
//! * En passant capturing
//! * Promotion
//! * Checks
//! * Checkmates
//! * Stalemate
//! * Draws via the 50-move rule

mod basic_moves;
mod semilegal_moves;

#[cfg(test)]
mod test;

