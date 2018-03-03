//! Chess rules module.
//!
//! This module contains rules around the movement of pieces and
//! general victory/defeat conditions.
//!
//! This includes:
//! * Which pieces can move where (move generation)
//! * Castling
//! * En passant capturing
//! * Promotion
//! * Checks
//! * Checkmates
//! * Stalemate
//! * Draws via the 50-move rule

mod basic_moves;
mod semilegal_moves;
mod movement;

#[cfg(test)]
mod test;
