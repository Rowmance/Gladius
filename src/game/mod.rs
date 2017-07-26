//! Chess game module, primarily revolving around the rules.
//!
//! This module contains rules around the movement of pieces and
//! general victory/defeat conditions.
//!
//! This includes:
//! * Which pieces can move where
//! * Only the player whose turn it is can move a piece
//! * Castling
//! * En passant capturing
//! * Promotion
//! * Checks
//! * Checkmates
//! * Stalemate
//! * Draws via the 50-move rule

mod valid_moves;

#[cfg(test)]
mod test;
