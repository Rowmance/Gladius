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

pub mod basic_moves;
pub mod semilegal_moves;
pub mod move_application;
pub mod game_state;
pub mod player_board;
pub mod castle_rights;

#[cfg(test)]
mod test;
