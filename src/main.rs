//! Gladius chess engine.
//! Written in Rust.
//! Work in progress.

#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces, unused_qualifications)]

pub mod board;

extern crate bit_reverse;
extern crate option_filter;

#[doc(hidden)]
#[macro_use]
extern crate derive_builder;

/// The main method.
fn main() {}
