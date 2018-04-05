//! Generates legal moves.

use rules::game_state::GameState;
use rules::move_application::Move;
use board::piece::Piece;

impl GameState {
    /// Returns a list of legal moves for the state.
    pub fn legal_moves(&self) -> Vec<Move> {
        let own_board = self.player_board(self.player_turn);
        let opponent_board = self.player_board(self.player_turn.other());

        let mut attacks: Vec<_> = Piece::iter()
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
                            piece: *piece,
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

        let mut moves: Vec<_> = Piece::iter()
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
                        .map(|a| {
                            println!("{}", a);
                            a
                        })
                        .map(move |target| Move {
                            piece: *piece,
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

        // TODO: Castles, en passant captures, promotions
        // TODO: don't return checks...

        attacks.append(&mut moves);
        attacks
    }
}
