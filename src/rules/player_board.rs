//! The positions of the pieces for a player

use board::bitboard;
use board::bitboard::BitBoard;
use board::piece::Piece;
use board::player::Player;

/// Represents the positions of the pieces for a player
#[derive(Clone, Debug, Copy, Eq, PartialEq, Withers)]
pub struct PlayerBoard {
    /// The pawns.
    pub pawns: BitBoard,

    /// The rooks.
    pub rooks: BitBoard,

    /// The knights.
    pub knights: BitBoard,

    /// The bishops.
    pub bishops: BitBoard,

    /// The queens.
    pub queens: BitBoard,

    /// The king.
    pub king: BitBoard,
}

impl PlayerBoard {
    /// Returns a copy of the instance with the given piece set.
    pub fn with_piece(self, piece: Piece, bitboard: BitBoard) -> Self {
        match piece {
            Piece::Pawn => self.with_pawns(bitboard),
            Piece::Rook => self.with_rooks(bitboard),
            Piece::Knight => self.with_knights(bitboard),
            Piece::Bishop => self.with_bishops(bitboard),
            Piece::Queen => self.with_queens(bitboard),
            Piece::King => self.with_king(bitboard),
        }
    }

    /// Returns the positions of the given piece on the board.
    pub fn piece(&self, piece: Piece) -> BitBoard {
        match piece {
            Piece::Pawn => self.pawns,
            Piece::Rook => self.rooks,
            Piece::Knight => self.knights,
            Piece::Bishop => self.bishops,
            Piece::Queen => self.queens,
            Piece::King => self.king,
        }
    }

    /// Returns the combined squares of all the players pieces.
    pub fn all(&self) -> BitBoard {
        self.pawns | self.knights | self.rooks | self.bishops | self.queens | self.king
    }

    // --------------------------
    /// Returns the start position for the given player.
    pub fn start_position(player: Player) -> Self {
        match player {
            Player::White => PlayerBoard {
                pawns: bitboard::WHITE_START_PAWNS,
                rooks: bitboard::WHITE_START_ROOKS,
                knights: bitboard::WHITE_START_KNIGHTS,
                bishops: bitboard::WHITE_START_BISHOPS,
                queens: bitboard::WHITE_START_QUEENS,
                king: bitboard::WHITE_START_KINGS,
            },
            Player::Black => PlayerBoard {
                pawns: bitboard::BLACK_START_PAWNS,
                rooks: bitboard::BLACK_START_ROOKS,
                knights: bitboard::BLACK_START_KNIGHTS,
                bishops: bitboard::BLACK_START_BISHOPS,
                queens: bitboard::BLACK_START_QUEENS,
                king: bitboard::BLACK_START_KINGS,
            },
        }
    }
}

impl Default for PlayerBoard {
    fn default() -> Self {
        PlayerBoard {
            pawns: BitBoard::empty(),
            rooks: BitBoard::empty(),
            knights: BitBoard::empty(),
            bishops: BitBoard::empty(),
            queens: BitBoard::empty(),
            king: BitBoard::empty(),
        }
    }
}
