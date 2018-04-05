use rules::game_state::GameState;
use board::piece::Piece;

#[test]
fn starting_position() {
    let state = GameState::start_position();
    let all_moves = state.legal_moves();

    let pawn_moves: Vec<_> = all_moves
        .iter()
        .filter(|m| m.piece == Piece::Pawn)
        .collect();

    let knight_moves: Vec<_> = all_moves
        .iter()
        .filter(|m| m.piece == Piece::Knight)
        .collect();

    assert_eq!(all_moves.len(), 20);
    assert_eq!(pawn_moves.len(), 16);
    assert_eq!(knight_moves.len(), 4);
}
