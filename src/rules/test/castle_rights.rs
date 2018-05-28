use rules::castle_rights::CastleRights;

#[test]
fn castle_rights() {
    assert_eq!(CastleRights::None.is_queen_side_available(), false);
    assert_eq!(CastleRights::None.is_king_side_available(), false);

    assert_eq!(CastleRights::Both.is_queen_side_available(), true);
    assert_eq!(CastleRights::Both.is_king_side_available(), true);

    assert_eq!(CastleRights::QueenSide.is_queen_side_available(), true);
    assert_eq!(CastleRights::QueenSide.is_king_side_available(), false);

    assert_eq!(CastleRights::KingSide.is_queen_side_available(), false);
    assert_eq!(CastleRights::KingSide.is_king_side_available(), true);

    assert_eq!(CastleRights::Both.without_king_side(), CastleRights::QueenSide);
    assert_eq!(CastleRights::Both.without_queen_side(), CastleRights::KingSide);

    assert_eq!(CastleRights::QueenSide.without_queen_side(), CastleRights::None);
    assert_eq!(CastleRights::KingSide.without_king_side(), CastleRights::None);
}
