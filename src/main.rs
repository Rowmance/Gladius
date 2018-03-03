//! Gladius chess engine.
//! Written in Rust.
//! Work in progress.

#![deny(missing_docs, missing_debug_implementations, trivial_casts, unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications, unused_variables)]

pub mod board;
pub mod game;

extern crate bit_reverse;

#[doc(hidden)]
#[macro_use]
extern crate derive_builder;

/// The main method.
fn main() {}
