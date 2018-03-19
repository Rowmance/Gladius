//! The positions of the pieces for a player

use board::bitboard::BitBoard;
use board::bitboards;
use board::player::Player;
use board::piece::Piece;

/// Represents the positions of the pieces for a player
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
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
    /// Returns a copy of the instance with the given pawns.
    pub fn with_pawns(self, pawns: BitBoard) -> Self {
        Self { pawns, ..self }
    }

    /// Returns a copy of the instance with the given rooks.
    pub fn with_rooks(self, rooks: BitBoard) -> Self {
        Self { rooks, ..self }
    }

    /// Returns a copy of the instance with the given knights.
    pub fn with_knights(self, knights: BitBoard) -> Self {
        Self { knights, ..self }
    }

    /// Returns a copy of the instance with the given bishops.
    pub fn with_bishops(self, bishops: BitBoard) -> Self {
        Self { bishops, ..self }
    }

    /// Returns a copy of the instance with the given queens.
    pub fn with_queens(self, queens: BitBoard) -> Self {
        Self { queens, ..self }
    }

    /// Returns a copy of the instance with the given king position.
    pub fn with_king(self, king: BitBoard) -> Self {
        Self { king, ..self }
    }

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

    // --------------------------
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
                pawns: bitboards::WHITE_START_PAWNS,
                rooks: bitboards::WHITE_START_ROOKS,
                knights: bitboards::WHITE_START_KNIGHTS,
                bishops: bitboards::WHITE_START_BISHOPS,
                queens: bitboards::WHITE_START_QUEENS,
                king: bitboards::WHITE_START_KINGS,
            },
            Player::Black => PlayerBoard {
                pawns: bitboards::BLACK_START_PAWNS,
                rooks: bitboards::BLACK_START_ROOKS,
                knights: bitboards::BLACK_START_KNIGHTS,
                bishops: bitboards::BLACK_START_BISHOPS,
                queens: bitboards::BLACK_START_QUEENS,
                king: bitboards::BLACK_START_KINGS,
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
