use board::piece::Piece;

#[test]
fn piece() {
    let pieces: Vec<_> = Piece::iter().collect();
    assert_eq!(*pieces[0], Piece::Pawn);
    assert_eq!(*pieces[1], Piece::Rook);
    assert_eq!(*pieces[2], Piece::Knight);
    assert_eq!(*pieces[3], Piece::Bishop);
    assert_eq!(*pieces[4], Piece::Queen);
    assert_eq!(*pieces[5], Piece::King);
}
