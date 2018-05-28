//! The complete state of a chess board.

use std::char;
use std::default::Default;
use std::fmt::{Display, Formatter, Result};

use board::file::File;
use board::piece::Piece;
use board::player::Player;
use board::rank::Rank;
use board::square::Square;
use rules::castle_rights::CastleRights;
use rules::player_board::PlayerBoard;

/// Represents a complete state of a chess board.
#[derive(Clone, Debug, Eq, PartialEq, Withers)]
pub struct GameState {
    /// The white player board.
    pub white_board: PlayerBoard,

    /// The black player board.
    pub black_board: PlayerBoard,

    /// The player whose turn it is.
    pub player_turn: Player,

    /// The square an en-passant capture is available on, if any.
    pub en_passant: Option<Square>,

    /// White players castle rights.
    pub white_castle_rights: CastleRights,

    /// Black players castle rights.
    pub black_castle_rights: CastleRights,

    /// The number of half-turns since the last capture or pawn advance.
    pub draw_plies: usize,

    /// The number of full turns elapsed.
    pub full_turns: usize,
}

impl GameState {
    /// Returns the standard chess starting board.
    pub fn start_position() -> Self {
        Self {
            white_board: PlayerBoard::start_position(Player::White),
            black_board: PlayerBoard::start_position(Player::Black),
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::Both,
            draw_plies: 0,
            full_turns: 0,
        }
    }

    /// Returns the player board of the given player.
    pub fn player_board(&self, player: Player) -> PlayerBoard {
        match player {
            Player::White => self.white_board,
            Player::Black => self.black_board,
        }
    }

    /// Returns the castle rights of the given player.
    pub fn castle_rights(&self, player: Player) -> CastleRights {
        match player {
            Player::White => self.white_castle_rights,
            Player::Black => self.black_castle_rights,
        }
    }

    /// Sets the castle rights for the given player
    pub fn set_castle_rights(&mut self, player: Player, castle_rights: CastleRights) {
        match player {
            Player::White => self.white_castle_rights = castle_rights,
            Player::Black => self.black_castle_rights = castle_rights,
        };
    }

    /// Parses the given FEN.
    pub fn parse_fen(_fen: &str) -> Self {
        // TODO.
        // Iterate over all squares and traverse the string alongside that
        unimplemented!()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            white_board: PlayerBoard::default(),
            black_board: PlayerBoard::default(),
            player_turn: Player::White,
            en_passant: None,
            white_castle_rights: CastleRights::Both,
            black_castle_rights: CastleRights::Both,
            draw_plies: 0,
            full_turns: 0,
        }
    }
}

//---------------------------------------------------------------------------
// Display
impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Gets the character for a given piece.
        fn piece_char(piece: Piece, player: Player) -> char {
            let val = match player {
                Player::White => 0x2654,
                Player::Black => 0x265A,
            } + match piece {
                Piece::King => 0,
                Piece::Queen => 1,
                Piece::Rook => 2,
                Piece::Bishop => 3,
                Piece::Knight => 4,
                Piece::Pawn => 5,
            };
            char::from_u32(val).unwrap()
        }

        let mut str = String::from("\n");

        str.push_str(format!("{} to move\n", self.player_turn).as_str());
        str.push_str("  +-+-+-+-+-+-+-+-+-+\n");
        for &rank in Rank::iter().rev() {
            str.push_str(format!("{} | ", rank).as_str());

            for &file in File::iter() {
                let square = Square::from_coordinates(file, rank);

                let char = self.en_passant
                    .and_then(|sq| if sq == square { Some(sq) } else { None })
                    .map(|_| 'e')
                    .or_else(|| {
                        if self.white_board.pawns.is_square_set(square) {
                            Some((Piece::Pawn, Player::White))
                        } else if self.white_board.rooks.is_square_set(square) {
                            Some((Piece::Rook, Player::White))
                        } else if self.white_board.knights.is_square_set(square) {
                            Some((Piece::Knight, Player::White))
                        } else if self.white_board.bishops.is_square_set(square) {
                            Some((Piece::Bishop, Player::White))
                        } else if self.white_board.queens.is_square_set(square) {
                            Some((Piece::Queen, Player::White))
                        } else if self.white_board.king.is_square_set(square) {
                            Some((Piece::King, Player::White))
                        } else if self.black_board.pawns.is_square_set(square) {
                            Some((Piece::Pawn, Player::Black))
                        } else if self.black_board.rooks.is_square_set(square) {
                            Some((Piece::Rook, Player::Black))
                        } else if self.black_board.knights.is_square_set(square) {
                            Some((Piece::Knight, Player::Black))
                        } else if self.black_board.bishops.is_square_set(square) {
                            Some((Piece::Bishop, Player::Black))
                        } else if self.black_board.queens.is_square_set(square) {
                            Some((Piece::Queen, Player::Black))
                        } else if self.black_board.king.is_square_set(square) {
                            Some((Piece::King, Player::Black))
                        } else {
                            None
                        }.map(|(piece, player)| piece_char(piece, player))
                    })
                    .unwrap_or(' ');
                str.push(char);
                str.push(' ');
            }

            str.push_str("|\n");
        }
        str.push_str("  +-+-+-+-+-+-+-+-+-+\n");
        str.push_str("    A B C D E F G H  ");
        write!(f, "{}", str)
    }
}
