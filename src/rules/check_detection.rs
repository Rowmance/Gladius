//! Check and checkmate detection.

use rules::game_state::GameState;
use board::piece::Piece;

impl GameState {
    // Returns true if the player whose turn it is is in check.
    pub fn is_check(&self) -> bool {
        let own_board = self.player_board(self.player_turn);
        let king_square = own_board.king.to_square();
        let own_pieces = own_board.all();
        let opponent_pieces = self.player_board(self.player_turn.other());

        // if for example, bishop attacks from the king contains a bishop,
        // then a bishop can attack the king. Repeat this for all pieces.
        Piece::iter().any(|p| {
            !(p.attacks(
                king_square,
                self.player_turn,
                own_pieces,
                opponent_pieces.all(),
            ) & opponent_pieces.piece(p))
                .is_empty()
        })
    }

    /// Returns true if the player whose turn it is has been mated.
    pub fn is_mate(&self) -> bool {
        self.legal_moves().is_empty()
    }
}
