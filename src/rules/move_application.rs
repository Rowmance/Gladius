//! A movement of a piece

use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::square::Square;
use rules::castle_rights::CastleRights;
use rules::game_state::GameState;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result;

/// Represents the direction of a castle move
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CastleMove {
    /// A king-side castle.
    KingSide,

    /// A queen-side castle.
    QueenSide,
}

impl Display for CastleMove {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
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
        let piece = match self.piece {
            Piece::Pawn => "",
            Piece::Rook => "R",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Queen => "Q",
            Piece::King => "K",
        };
        write!(
            f,
            "{}: {} {} {}",
            piece,
            self.origin,
            if self.capture { "X" } else { "->" },
            self.target
        ) // TODO
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
    pub fn apply_move(&self, mv: &Move) -> Self {
        debug_assert!({
            let result = self.validate(&mv);
            if let Err(ref message) = result {
                println!("{}", message);
            }
            result.is_ok()
        });

        let mut new_state = self.clone();
        new_state.en_passant = None;

        // Castling
        if let Some(castle_move) = mv.castle {
            new_state.apply_castle(castle_move);
        }
        // Promotion
        else if mv.promotion.is_some() {
            new_state.apply_promotion(&mv);
        }
        // All other non-capture moves
        else if !mv.capture {
            new_state.apply_non_capture(&mv);
        }
        // En passant
        else if mv.en_passant {
            new_state.apply_en_passant(&mv);
        }
        // All other capture moves
        else {
            new_state.apply_capture(mv);
        }

        if self.player_turn == Player::Black {
            new_state.full_turns += 1;
        }

        new_state.player_turn = match self.player_turn {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };

        if mv.capture || mv.piece == Piece::Pawn {
            new_state.draw_plies = 0;
        } else {
            new_state.draw_plies += 1;
        }

        if mv.piece == Piece::King {
            new_state.set_castle_rights(self.player_turn, CastleRights::None);
        } else if mv.piece == Piece::Rook {
            let castle_rights = self.castle_rights(self.player_turn);
            if castle_rights.is_king_side_available() && mv.origin.file() == File::H {
                new_state.set_castle_rights(self.player_turn, castle_rights.without_king_side());
            } else if castle_rights.is_queen_side_available() && mv.origin.file() == File::A {
                new_state.set_castle_rights(self.player_turn, castle_rights.without_queen_side());
            }
        }

        new_state
    }

    // ----------------------------------------------------------------
    // returns None if the move and board are valid, or an error
    fn validate(&self, mv: &Move) -> Result<(), String> {
        // valid origin
        if !match self.player_turn {
            Player::White => self.white_board.piece(mv.piece).is_square_set(mv.origin),
            Player::Black => self.black_board.piece(mv.piece).is_square_set(mv.origin),
        } {
            return Err(format!(
                "The moved {} from square {} for player {} was not in place",
                mv.piece, mv.origin, self.player_turn
            ));
        }

        // validate target - en passant doesn't capture where a piece is
        if mv.capture && !mv.en_passant {
            if !match self.player_turn {
                Player::White => self.black_board.all().is_square_set(mv.target),
                Player::Black => self.white_board.all().is_square_set(mv.target),
            } {
                return Err(format!(
                    "The captured piece from square {} for player {} was not in place",
                    mv.target, self.player_turn
                ));
            }
        } else {
            let all_pieces = self.black_board.all() | self.white_board.all();
            if all_pieces.is_square_set(mv.target) {
                return Err(format!(
                    "The target square {} for player {} isn't empty",
                    mv.target, self.player_turn
                ));
            }
        };

        // ensure the promotion is to a valid square and piece
        if let Some(promo_piece) = mv.promotion {
            if promo_piece == Piece::Pawn {
                return Err("Cannot promote to a pawn".to_string());
            }

            if match self.player_turn {
                Player::White => mv.origin.rank() != Rank::Seven || mv.target.rank() != Rank::Eight,
                Player::Black => mv.origin.rank() != Rank::Two || mv.target.rank() != Rank::One,
            } {
                return Err(format!(
                    "Promotion move from {} to {} is invalid",
                    mv.origin, mv.target
                ));
            }
        };

        // ensure that a castle has the correct rights and that the correct spaces are empty
        if let Some(castle_move) = mv.castle {
            let castle_rights = self.castle_rights(self.player_turn);
            if castle_move == CastleMove::KingSide && !castle_rights.is_king_side_available() {
                return Err(format!(
                    "{} is invalid with {} castling rights",
                    castle_move, castle_rights
                ));
            }
            if castle_move == CastleMove::QueenSide && !castle_rights.is_queen_side_available() {
                return Err(format!(
                    "{} is invalid with {} castling rights",
                    castle_move, castle_rights
                ));
            }

            let all = self.black_board.all() | self.white_board.all();
            let king_rank = match self.player_turn {
                Player::White => Rank::One,
                Player::Black => Rank::Eight,
            };

            if castle_move == CastleMove::KingSide {
                if all.is_square_set(Square::from_coordinates(File::G, king_rank))
                    || all.is_square_set(Square::from_coordinates(File::F, king_rank))
                {
                    return Err(format!(
                        "{} is invalid as there are pieces in the way",
                        castle_move
                    ));
                }
            }

            if castle_move == CastleMove::QueenSide {
                if all.is_square_set(Square::from_coordinates(File::B, king_rank))
                    || all.is_square_set(Square::from_coordinates(File::C, king_rank))
                    || all.is_square_set(Square::from_coordinates(File::D, king_rank))
                {
                    return Err(format!(
                        "{} is invalid as there are pieces in the way",
                        castle_move
                    ));
                }
            }
        }

        // ensure en passant is allowed and on the correct square
        if mv.en_passant {
            if let Some(target) = self.en_passant {
                if target != mv.target {
                    return Err(format!(
                        "Cannot en-passant capture on {} when en-passant square is {}",
                        mv.target, target
                    ));
                }
            } else {
                return Err(format!(
                    "Cannot en-passant capture on {} when there is no en-passant available",
                    mv.target
                ));
            }
        }

        // ensure pawn moves to the back ranks are promotions
        if mv.piece == Piece::Pawn
            && (mv.target.rank() == Rank::Eight || mv.target.rank() == Rank::One)
            && mv.promotion.is_none()
        {
            println!("{}", mv);
            return Err(format!("Pawn move to {} must be a promotion", mv.target));
        }

        if mv.capture {
            let own_pieces = self.player_board(self.player_turn).all();
            let mut opponent_pieces = self.player_board(self.player_turn.other()).all();
            if mv.piece == Piece::Pawn {
                if let Some(en_passant) = self.en_passant {
                    // imitate attackable square for pawn
                    opponent_pieces = opponent_pieces.set_square(en_passant);
                }
            }
            let valid_captures =
                mv.piece
                    .attacks(mv.origin, self.player_turn, own_pieces, opponent_pieces);
            if !valid_captures.is_square_set(mv.target) {
                return Err(format!(
                    "{} capture from {} to {} is not valid",
                    mv.piece, mv.origin, mv.target
                ));
            }
        } else if mv.castle.is_none() {
            let blockers = self.white_board.all() | self.black_board.all();
            let valid_moves = mv.piece.moves(mv.origin, self.player_turn, blockers);
            if !valid_moves.is_square_set(mv.target) {
                return Err(format!(
                    "{} move from {} to {} is not valid",
                    mv.piece, mv.origin, mv.target
                ));
            }
        }

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
    fn apply_promotion(&mut self, mv: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board
                    .with_pawns(self.white_board.pawns.unset_square(mv.origin))
                    .with_piece(
                        mv.promotion
                            .expect("apply_promotion called without promotion piece"),
                        mv.target.to_bitboard(),
                    )
            }
            Player::Black => {
                self.black_board = self.black_board
                    .with_pawns(self.black_board.pawns.unset_square(mv.origin))
                    .with_piece(
                        mv.promotion
                            .expect("apply_promotion called without promotion piece"),
                        mv.target.to_bitboard(),
                    )
            }
        }
    }

    /// Applies a non-capturing move to the state.
    fn apply_non_capture(&mut self, mv: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board.with_piece(
                    mv.piece,
                    self.white_board
                        .piece(mv.piece)
                        .unset_square(mv.origin)
                        .set_square(mv.target),
                );

                if mv.piece == Piece::Pawn && mv.origin.rank() == Rank::Two
                    && mv.target.rank() == Rank::Four
                {
                    self.en_passant = Some(Square::from_coordinates(mv.origin.file(), Rank::Three))
                }
            }
            Player::Black => {
                self.black_board = self.black_board.with_piece(
                    mv.piece,
                    self.black_board
                        .piece(mv.piece)
                        .unset_square(mv.origin)
                        .set_square(mv.target),
                );
                if mv.piece == Piece::Pawn && mv.origin.rank() == Rank::Seven
                    && mv.target.rank() == Rank::Five
                {
                    self.en_passant = Some(Square::from_coordinates(mv.origin.file(), Rank::Six))
                }
            }
        }
    }

    /// Applies an en-passant capture to the state.
    fn apply_en_passant(&mut self, mv: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board.with_pawns(
                    self.white_board
                        .pawns
                        .unset_square(mv.origin)
                        .set_square(mv.target),
                );
                let target =
                    Square::from_coordinates(mv.target.file(), mv.target.rank().prev().unwrap());
                self.black_board = self.black_board
                    .with_pawns(self.black_board.pawns.unset_square(target));
            }
            Player::Black => {
                self.black_board = self.black_board.with_pawns(
                    self.black_board
                        .pawns
                        .unset_square(mv.origin)
                        .set_square(mv.target),
                );
                let target =
                    Square::from_coordinates(mv.target.file(), mv.target.rank().next().unwrap());
                self.white_board = self.white_board
                    .with_pawns(self.white_board.pawns.unset_square(target));
            }
        }
    }

    /// Applies an ordinary capture move to the state.
    fn apply_capture(&mut self, mv: &Move) {
        match self.player_turn {
            Player::White => {
                self.white_board = self.white_board.with_piece(
                    mv.piece,
                    self.white_board
                        .piece(mv.piece)
                        .unset_square(mv.origin)
                        .set_square(mv.target),
                );
                self.black_board = match self.black_board {
                    board if board.pawns.is_square_set(mv.target) => {
                        board.with_pawns(board.pawns.unset_square(mv.target))
                    }
                    board if board.rooks.is_square_set(mv.target) => {
                        board.with_rooks(board.rooks.unset_square(mv.target))
                    }
                    board if board.knights.is_square_set(mv.target) => {
                        board.with_knights(board.knights.unset_square(mv.target))
                    }
                    board if board.bishops.is_square_set(mv.target) => {
                        board.with_bishops(board.bishops.unset_square(mv.target))
                    }
                    board => board.with_queens(board.queens.unset_square(mv.target)),
                }
            }
            Player::Black => {
                self.black_board = self.black_board.with_piece(
                    mv.piece,
                    self.black_board
                        .piece(mv.piece)
                        .unset_square(mv.origin)
                        .set_square(mv.target),
                );
                self.white_board = match self.white_board {
                    board if board.pawns.is_square_set(mv.target) => {
                        board.with_pawns(board.pawns.unset_square(mv.target))
                    }
                    board if board.rooks.is_square_set(mv.target) => {
                        board.with_rooks(board.rooks.unset_square(mv.target))
                    }
                    board if board.knights.is_square_set(mv.target) => {
                        board.with_knights(board.knights.unset_square(mv.target))
                    }
                    board if board.bishops.is_square_set(mv.target) => {
                        board.with_bishops(board.bishops.unset_square(mv.target))
                    }
                    board => board.with_queens(board.queens.unset_square(mv.target)),
                }
            }
        }
    }
}
