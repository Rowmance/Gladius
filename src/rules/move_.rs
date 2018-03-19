//! A movement of a piece

use board::square::Square;
use board::piece::Piece;
use board::game_state::GameState;
use std::fmt::{Display, Formatter, Result};
use board::player::Player;
use board::castle_rights::CastleRights;
use board::file::File;
use board::rank::Rank;

/// Represents the direction of a castle move
#[derive(Clone, Debug)]
pub enum CastleMove {
    /// A king-side castle.
    KingSide,

    /// A queen-side castle.
    QueenSide,
}

/// A movement of a piece (a 'move')
#[derive(Clone, Debug)]
pub struct Move {
    /// The type of the piece which moved, king if castle.
    pub piece: Piece,

    /// The square the piece moved from.
    pub origin: Square,

    /// The square the piece moved to.
    pub target: Square,

    /// True if the move was a capture.
    pub capture: bool,

    /// True if the capture was en-passant.
    pub en_passant: bool,

    /// Present and containing the target piece if the move was a promotion.
    pub promotion: Option<Piece>,

    /// Present and contains the castle direction if the move was a castle.
    pub castle: Option<CastleMove>,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = String::from("\n");

        write!(f, "{}", str)
    }
}

impl GameState {
    /// Apply a given move to the board.
    ///
    /// Returns a game state modified based on the move provided
    /// - If the move is a capture, removes the opponent piece and moves the
    ///   capturing piece to the captured square.
    /// - If the move is just a movement, the piece is simply moved.
    /// - En passant captures remove the captured pawn and move the capturing
    ///   pawn to the space behind.
    /// - A promotion will remove the pawn and add the respective piece
    /// - A castle will move the pieces appropriately and set the castle rights
    ///   options.
    ///
    /// This method also:
    /// - Sets the player turn to the other player.
    /// - Resets the en-passant square.
    /// - Increases the full turns count.
    /// - Increases the draw plies count if necessary.
    /// - Updates the castle rights if a rook or king move
    ///
    /// Invalid game states or moves will give undefined behaviour.
    pub fn apply_move(&self, move_: Move) -> Self {
        let mut new_state = self.clone();

        new_state.player_turn = match self.player_turn {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };

        if self.player_turn == Player::Black {
            new_state.full_turns += 1;
        }

        if move_.capture || move_.piece == Piece::Pawn {
            new_state.draw_plies = 0;
        } else {
            new_state.draw_plies += 1;
        }

        new_state.en_passant = None;

        // Castling
        if let Some(castle_move) = move_.castle {
            match self.player_turn {
                Player::White => {
                    match castle_move {
                        CastleMove::KingSide => {
                            new_state.white_board = self.white_board
                                .with_king(Square::new(6).to_bitboard()) //G1
                                .with_rooks(self.white_board.rooks
                                    .unset_square(Square::new(7)) // H1
                                    .set_square(Square::new(5))); // F1
                        }
                        CastleMove::QueenSide => {
                            new_state.white_board = self.white_board
                                .with_king(Square::new(2).to_bitboard()) // C1
                                .with_rooks(self.white_board.rooks
                                    .unset_square(Square::new(0)) // A1
                                    .set_square(Square::new(3))); // D1
                        }
                    }
                    new_state.white_castle_rights = CastleRights::None;
                }
                Player::Black => {
                    match castle_move {
                        CastleMove::KingSide => {
                            new_state.black_board = self.black_board
                                .with_king(Square::new(62).to_bitboard()) // G8
                                .with_rooks(self.black_board.rooks
                                    .unset_square(Square::new(63)) // H8
                                    .set_square(Square::new(61))); // F8
                        }
                        CastleMove::QueenSide => {
                            new_state.black_board = self.black_board
                                .with_king(Square::new(58).to_bitboard()) // C8
                                .with_rooks(self.black_board.rooks
                                    .unset_square(Square::new(56)) // A8
                                    .set_square(Square::new(59))); // D8
                        }
                    }
                    new_state.black_castle_rights = CastleRights::None;
                }
            };
            return new_state;
        }

        if let Some(promo) = move_.promotion {
            match self.player_turn {
                Player::White => {
                    new_state.white_board = self.white_board
                        .with_pawns(self.white_board.pawns.unset_square(move_.origin))
                        .with_piece(promo, move_.target)
                }
                Player::Black => {
                    new_state.black_board = self.black_board
                        .with_pawns(self.black_board.pawns.unset_square(move_.origin))
                        .with_piece(promo, move_.target)
                }
            }
            return new_state;
        }

        unimplemented!()
    }
}
