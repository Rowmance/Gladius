//! Gladius chess engine.
//! Written in Rust.
//! Work in progress.

#![deny(
    missing_docs, missing_debug_implementations, trivial_casts, unsafe_code, unstable_features,
    unused_import_braces, unused_qualifications, unused_variables
)]

pub mod board;
pub mod engine;
pub mod logger;
pub mod rules;

extern crate bit_reverse;

#[doc(hidden)]
#[macro_use]
extern crate withers_derive;

#[macro_use]
extern crate log;
extern crate fern;

extern crate chrono;
extern crate futures;

/// The main method.
fn main() -> Result<(), ()> {
    logger::setup().map_err(|_| ());
    Ok(())
}
