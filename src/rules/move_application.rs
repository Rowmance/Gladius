//! A movement of a piece

use board::square::Square;
use board::piece::Piece;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;
use board::player::Player;
use board::rank::Rank;
use rules::game_state::GameState;
use rules::castle_rights::CastleRights;

/// Represents the direction of a castle move
#[derive(Clone, Copy, Debug)]
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
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let str = String::from("\n");
        // TODO
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
    /// - Sets or resets the en-passant square.
    /// - Increases the full turns count.
    /// - Increases the draw plies count if necessary.
    /// - Updates the castle rights if a rook or king move
    ///
    /// Invalid game states or moves will give undefined behaviour.
    pub fn apply_move(&self, move_: &Move) -> Self {
        // TODO is there a nicer way of doing this? debug-mode compilation should be a thing
        debug_assert!({
            let result = self.validate(&move_);
            if let Err(ref message) = result {
                println!("{}", message);
            }
            result.is_ok()
        });

        let mut new_state = self.clone();
        new_state.en_passant = None;

        // Castling
        if let Some(castle_move) = move_.castle {
            new_state.apply_castle(castle_move);
        }
        // Promotion
        else if move_.promotion.is_some() {
            new_state.apply_promotion(&move_);
        }
        // All other non-capture moves
        else if !move_.capture {
            new_state.apply_non_capture(&move_);
        }
        // En passant
        else if move_.en_passant {
            new_state.apply_en_passant(&move_);
        }
        // All other capture moves
        else {
            new_state.apply_capture(move_);
        }

        if self.player_turn == Player::Black {
            new_state.full_turns += 1;
        }

        new_state.player_turn = match self.player_turn {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };

        if move_.capture || move_.piece == Piece::Pawn {
            new_state.draw_plies = 0;
        } else {
            new_state.draw_plies += 1;
        }

        new_state
    }

    // ----------------------------------------------------------------
    // returns None if the move and board are valid, or an error
    fn validate(&self, move_: &Move) -> Result<(), String> {
        // valid origin
        if !match self.player_turn {
            Player::White => self.white_board
                .piece(&move_.piece)
                .is_square_set(move_.origin),
            Player::Black => self.black_board
                .piece(&move_.piece)
                .is_square_set(move_.origin),
        } {
            return Err(format!(
                "The moved {} from square {} for player {} was not in place",
                move_.piece, move_.origin, self.player_turn
            ));
        }

        // validate target - en passant doesn't capture where a piece is
        if move_.capture && !move_.en_passant {
            if !match self.player_turn {
                Player::White => self.black_board.all().is_square_set(move_.target),
                Player::Black => self.white_board.all().is_square_set(move_.target),
            } {
                return Err(format!(
                    "The captured piece from square {} for player {} was not in place",
                    move_.target, self.player_turn
                ));
            }
        } else {
            let all_pieces = self.black_board.all() | self.white_board.all();
            if all_pieces.is_square_set(move_.target) {
                return Err(format!(
                    "The target square {} for player {} isn't empty",
                    move_.target, self.player_turn
                ));
            }
        };

        if move_.promotion.is_some() {
            if match self.player_turn {
                Player::White => move_.origin.rank() != Rank::Seven || move_.target.rank() != Rank::Eight,
                Player::Black => move_.origin.rank() != Rank::Two || move_.target.rank() != Rank::One,
            } {
                return Err(format!(
                    "Promotion move from {} to {} is invalid",
                    move_.origin, move_.target
                ));
            }
        };

        // TODO: Valid castle and en passant
        // TODO: Make sure piece can move to the target square from the origin

        Ok(())
    }

    // ----------------------------------------------------------------
    /// Applies castling to the state.
    fn apply_castle(&mut self, castle_move: CastleMove) {
        match self.player_turn {
            Player::White => {
                match castle_move {
                    CastleMove::KingSide => {
                        self.white_board = self.white_board // TODO: const these
                            .with_king(Square::new(6).to_bitboard()) //G1
                            .with_rooks(self.white_board.rooks
                                .unset_square(Square::new(7)) // H1
                                .set_square(Square::new(5))); // F1
                    }
                    CastleMove::QueenSide => {
                        self.white_board = self.white_board
                            .with_king(Square::new(2).to_bitboard()) // C1
                            .with_rooks(self.white_board.rooks
                                .unset_square(Square::new(0)) // A1
                                .set_square(Square::new(3))); // D1
                    }
                }
                self.white_castle_rights = CastleRights::None;
            }
            Player::Black => {
                match castle_move {
                    CastleMove::KingSide => {
                        self.black_board = self.black_board
                            .with_king(Square::new(62).to_bitboard()) // G8
                            .with_rooks(self.black_board.rooks
                                .unset_square(Square::new(63)) // H8
                                .set_square(Square::new(61))); // F8
                    }
                    CastleMove::QueenSide => {
                        self.black_board = self.black_board
                            .with_king(Square::new(58).to_bitboard()) // C8
                            .with_rooks(self.black_board.rooks
                                .unset_square(Square::new(56)) // A8
                                .set_square(Square::new(59))); // D8
                    }
                }
                self.black_castle_rights = CastleRights::None;
            }
        };
    }

    /// Applies a promoting move to the state.
    fn apply_promotion(&mut self, move_: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board
                    .with_pawns(self.white_board.pawns.unset_square(move_.origin))
                    .with_piece(
                        move_
                            .promotion
                            .expect("apply_promotion called without promotion piece"),
                        move_.target.to_bitboard(),
                    )
            }
            Player::Black => {
                self.black_board = self.black_board
                    .with_pawns(self.black_board.pawns.unset_square(move_.origin))
                    .with_piece(
                        move_
                            .promotion
                            .expect("apply_promotion called without promotion piece"),
                        move_.target.to_bitboard(),
                    )
            }
        }
    }

    /// Applies a non-capturing move to the state.
    fn apply_non_capture(&mut self, move_: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board.with_piece(
                    move_.piece,
                    self.white_board
                        .piece(&move_.piece)
                        .unset_square(move_.origin)
                        .set_square(move_.target),
                );

                if move_.piece == Piece::Pawn && move_.origin.rank() == Rank::Two && move_.target.rank() == Rank::Four {
                    self.en_passant = Some(Square::from_coordinates(move_.origin.file(), Rank::Three))
                }
            }
            Player::Black => {
                self.black_board = self.black_board.with_piece(
                    move_.piece,
                    self.black_board
                        .piece(&move_.piece)
                        .unset_square(move_.origin)
                        .set_square(move_.target),
                );
                if move_.piece == Piece::Pawn && move_.origin.rank() == Rank::Seven && move_.target.rank() == Rank::Five
                {
                    self.en_passant = Some(Square::from_coordinates(move_.origin.file(), Rank::Six))
                }
            }
        }
    }

    /// Applies an en-passant capture to the state.
    fn apply_en_passant(&mut self, move_: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board.with_pawns(
                    self.white_board
                        .pawns
                        .unset_square(move_.origin)
                        .set_square(move_.target),
                );
                let target = Square::from_coordinates(move_.target.file(), move_.target.rank().prev().unwrap());
                self.black_board = self.black_board
                    .with_pawns(self.black_board.pawns.unset_square(target));
            }
            Player::Black => {
                self.black_board = self.black_board.with_pawns(
                    self.black_board
                        .pawns
                        .unset_square(move_.origin)
                        .set_square(move_.target),
                );
                let target = Square::from_coordinates(move_.target.file(), move_.target.rank().next().unwrap());
                self.white_board = self.white_board
                    .with_pawns(self.white_board.pawns.unset_square(target));
            }
        }
    }

    /// Applies an ordinary capture move to the state.
    fn apply_capture(&mut self, move_: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board.with_piece(
                    move_.piece,
                    self.white_board
                        .piece(&move_.piece)
                        .unset_square(move_.origin)
                        .set_square(move_.target),
                );
                self.black_board = match self.black_board {
                    board if board.pawns.is_square_set(move_.target) => {
                        board.with_pawns(board.pawns.unset_square(move_.target))
                    }
                    board if board.rooks.is_square_set(move_.target) => {
                        board.with_rooks(board.rooks.unset_square(move_.target))
                    }
                    board if board.knights.is_square_set(move_.target) => {
                        board.with_knights(board.knights.unset_square(move_.target))
                    }
                    board if board.bishops.is_square_set(move_.target) => {
                        board.with_bishops(board.bishops.unset_square(move_.target))
                    }
                    board => board.with_queens(board.queens.unset_square(move_.target)),
                }
            }
            Player::Black => {
                self.black_board = self.black_board.with_piece(
                    move_.piece,
                    self.black_board
                        .piece(&move_.piece)
                        .unset_square(move_.origin)
                        .set_square(move_.target),
                );
                self.white_board = match self.white_board {
                    board if board.pawns.is_square_set(move_.target) => {
                        board.with_pawns(board.pawns.unset_square(move_.target))
                    }
                    board if board.rooks.is_square_set(move_.target) => {
                        board.with_rooks(board.rooks.unset_square(move_.target))
                    }
                    board if board.knights.is_square_set(move_.target) => {
                        board.with_knights(board.knights.unset_square(move_.target))
                    }
                    board if board.bishops.is_square_set(move_.target) => {
                        board.with_bishops(board.bishops.unset_square(move_.target))
                    }
                    board => board.with_queens(board.queens.unset_square(move_.target)),
                }
            }
        }
    }
}
