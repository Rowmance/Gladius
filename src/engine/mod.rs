//! Chess engine module.
//!
//! This module contains the code for picking the best moves
//! out of all available moves.

#![allow(dead_code)]
#![allow(unused_must_use)]

pub mod calculator;
pub mod heuristic;

#[cfg(test)]
mod test;

// Alpha-beta pruning. This should be fairly easy, but needs a timeout and/or a maximum depth
// argument. Needs to be able to store best-so-far results too so if timeout or aborted,
// results are still available.

// Would be useful to memoize positions too, as no need to recalculate if opponent makes
// a given move.

// I have:
// game state
// list of available moves (to get game states too)
// Heuristic function for game state
// Check for mate/stalemate etc.

// I need:
// Function which ultimately returns list of move sequences to scores, given a depth
// An instance which takes lock on result whilst calculating, and then can be interrupted with pending results so far!
// don't worry about pruning just yet...
// don't worry about cancelling just yet either - just go until depth runs out.

// risks
// make sure that forced draw by repetition doesn't break things.

// next steps
// pruning
// cancellable future
// multithreading
// queue of search states so that checks can be explored in full before anything else?
// prioritise checks in general
// memoising?
