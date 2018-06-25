//! Generates legal moves.

use board::bitboard::BitBoard;
use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::square::Square;
use rules::castle_rights::CastleRights;
use rules::game_state::GameState;
use rules::move_application::{CastleMove, Move};
use std::iter;

impl GameState {
    /// Returns a list of legal moves for the state.
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let own_board = self.player_board(self.player_turn);
        let opponent_board = self.player_board(self.player_turn.other());

        let mut non_pawn_captures: Vec<_> = Piece::iter_non_pawn()
            .flat_map(|piece| {
                let piece_board = own_board.piece(piece);
                piece_board.iter().flat_map(move |square| {
                    piece
                        .attacks(
                            square,
                            self.player_turn,
                            own_board.all(),
                            opponent_board.all(),
                        )
                        .iter()
                        .map(move |target| Move {
                            piece,
                            target,
                            origin: square,
                            capture: true,
                            en_passant: false,
                            promotion: None,
                            castle: None,
                        })
                })
            })
            .collect();

        moves.append(&mut non_pawn_captures);

        let last_rank = match self.player_turn {
            Player::White => Rank::Eight,
            Player::Black => Rank::One,
        };

        // TODO: This is a bit of a shame - couldn't get it working otherwise though
        // (maybe impl traits will fix this?)
        fn process_pawn_moves(mv: Move, last_rank: Rank) -> Box<Iterator<Item = Move>> {
            if mv.target.rank() == last_rank {
                let iter = Piece::iter_pieces().map(move |piece| Move {
                    promotion: Some(piece),
                    ..mv
                });
                Box::new(iter)
            } else {
                Box::new(iter::once(mv))
            }
        }

        let mut pawn_captures = own_board
            .pawns
            .iter()
            .flat_map(move |square| {
                Piece::Pawn
                    .attacks(
                        square,
                        self.player_turn,
                        own_board.all(),
                        opponent_board.all(),
                    )
                    .iter()
                    .map(move |target| Move {
                        piece: Piece::Pawn,
                        target,
                        origin: square,
                        capture: true,
                        en_passant: false,
                        promotion: None,
                        castle: None,
                    })
            })
            .flat_map(move |mv| process_pawn_moves(mv, last_rank))
            .collect();

        moves.append(&mut pawn_captures);

        // pawns are special as they have promotions and en passant captures
        let mut non_pawn_moves: Vec<_> = Piece::iter_non_pawn()
            .flat_map(|piece| {
                let piece_board = own_board.piece(piece);
                piece_board.iter().flat_map(move |square| {
                    piece
                        .moves(
                            square,
                            self.player_turn,
                            own_board.all() | opponent_board.all(),
                        )
                        .iter()
                        .map(move |target| Move {
                            piece,
                            target,
                            origin: square,
                            capture: false,
                            en_passant: false,
                            promotion: None,
                            castle: None,
                        })
                })
            })
            .collect();

        moves.append(&mut non_pawn_moves);

        let mut pawn_moves: Vec<_> = own_board
            .pawns
            .iter()
            .flat_map(move |square| {
                Piece::Pawn
                    .moves(
                        square,
                        self.player_turn,
                        own_board.all() | opponent_board.all(),
                    )
                    .iter()
                    .map(move |target| Move {
                        piece: Piece::Pawn,
                        target,
                        origin: square,
                        capture: false,
                        en_passant: false,
                        promotion: None,
                        castle: None,
                    })
            })
            .flat_map(move |mv| process_pawn_moves(mv, last_rank))
            .collect();

        moves.append(&mut pawn_moves);

        if let Some(en_passant_target) = self.en_passant {
            let origin_rank = match self.player_turn {
                Player::White => Rank::Five,
                Player::Black => Rank::Four,
            };

            if let Some(origin_file) = en_passant_target.file().next() {
                let origin = Square::from_coordinates(origin_file, origin_rank);
                if own_board.pawns.is_square_set(origin) {
                    moves.push(Move {
                        origin,
                        piece: Piece::Pawn,
                        target: en_passant_target,
                        capture: true,
                        en_passant: true,
                        promotion: None,
                        castle: None,
                    })
                }
            }

            if let Some(origin_file) = en_passant_target.file().prev() {
                let origin = Square::from_coordinates(origin_file, origin_rank);
                if own_board.pawns.is_square_set(origin) {
                    moves.push(Move {
                        origin,
                        piece: Piece::Pawn,
                        target: en_passant_target,
                        capture: true,
                        en_passant: true,
                        promotion: None,
                        castle: None,
                    })
                }
            }
        }

        let castle_rights = self.castle_rights(self.player_turn);

        if castle_rights == CastleRights::None {
            let all_attacks = Piece::iter()
                .map(|piece| {
                    let piece_board = opponent_board.piece(piece);
                    piece_board
                        .iter()
                        .map(move |square| {
                            piece.attacks(
                                square,
                                self.player_turn,
                                own_board.all(),
                                opponent_board.all(),
                            )
                        })
                        .fold(BitBoard::empty(), |a, b| a ^ b)
                })
                .fold(BitBoard::empty(), |a, b| a ^ b);

            // TODO const these bitboards.
            // TODO names are awful here
            // prevent castling if king movement squares are being attacked
            let king_check_blockers = match self.player_turn {
                Player::White => BitBoard::empty()
                    .set_coordinate(File::E, Rank::One)
                    .set_coordinate(File::F, Rank::One)
                    .set_coordinate(File::G, Rank::One),
                Player::Black => BitBoard::empty()
                    .set_coordinate(File::E, Rank::Eight)
                    .set_coordinate(File::F, Rank::Eight)
                    .set_coordinate(File::G, Rank::Eight),
            };

            let queen_check_blockers = match self.player_turn {
                Player::White => BitBoard::empty()
                    .set_coordinate(File::E, Rank::One)
                    .set_coordinate(File::D, Rank::One)
                    .set_coordinate(File::C, Rank::One),
                Player::Black => BitBoard::empty()
                    .set_coordinate(File::E, Rank::Eight)
                    .set_coordinate(File::D, Rank::Eight)
                    .set_coordinate(File::C, Rank::Eight),
            };

            let can_ks_castle = (king_check_blockers & all_attacks).is_empty();
            let can_qs_castle = (queen_check_blockers & all_attacks).is_empty();

            if castle_rights.is_king_side_available() && can_ks_castle {
                let king_rank = match self.player_turn {
                    Player::White => Rank::One,
                    Player::Black => Rank::Eight,
                };
                let all = opponent_board.all() | own_board.all();
                if !all.is_square_set(Square::from_coordinates(File::G, king_rank))
                    && !all.is_square_set(Square::from_coordinates(File::F, king_rank))
                {
                    moves.push(Move {
                        piece: Piece::King,
                        origin: Square::from_coordinates(File::E, king_rank),
                        target: Square::from_coordinates(File::G, king_rank),
                        capture: false,
                        en_passant: false,
                        promotion: None,
                        castle: Some(CastleMove::KingSide),
                    })
                }
            }

            if castle_rights.is_queen_side_available() && can_qs_castle {
                let king_rank = match self.player_turn {
                    Player::White => Rank::One,
                    Player::Black => Rank::Eight,
                };
                let all = opponent_board.all() | own_board.all();
                if !all.is_square_set(Square::from_coordinates(File::B, king_rank))
                    && !all.is_square_set(Square::from_coordinates(File::C, king_rank))
                    && !all.is_square_set(Square::from_coordinates(File::D, king_rank))
                {
                    moves.push(Move {
                        piece: Piece::King,
                        origin: Square::from_coordinates(File::E, king_rank),
                        target: Square::from_coordinates(File::C, king_rank),
                        capture: false,
                        en_passant: false,
                        promotion: None,
                        castle: Some(CastleMove::QueenSide),
                    })
                }
            }
        }

        moves
            .into_iter()
            .filter(|mv| !self.apply_move(mv).is_check(self.player_turn))
            .collect::<Vec<Move>>()
    }
}
