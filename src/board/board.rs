//! The complete state of a chess board.

use std::collections::HashMap;
use std::fmt::{Formatter, Result, Display};
use std::default::Default;
use std::char;

use option_filter::OptionFilterExt;

use board::piece::Piece;
use board::bitboard::BitBoard;
use board::player::Player;
use board::square::Square;
use board::castle_rights::CastleRights;
use board::bitboards;
use board::rank::Rank;
use board::file::File;

/// Represents a complete state of a chess board.
#[derive(Clone, Builder)]
#[builder(pattern = "immutable")]
pub struct Board {
    /// A map of positions of each of the white player pieces.
    pub white_pieces: HashMap<Piece, BitBoard>,
    //TODO: Do immutable maps exist? How do they work?

    /// A map of positions of each of the black player pieces.
    pub black_pieces: HashMap<Piece, BitBoard>,

    /// The player whose turn it is.
    pub player_turn: Player,

    /// The square an en-passant capture is available on, if any.
    pub en_passant: Option<Square>,

    /// White players castle rights.
    pub white_castle_rights: CastleRights,

    /// Black players castle rights.
    pub black_castle_rights: CastleRights,

    /// The number of half-turns since the last capture or pawn advance.
    pub draw_half_turns: usize,

    /// The number of full turns elapsed.
    pub full_turns: usize
}

impl Board {
    /// Returns a builder
    pub fn builder() -> BoardBuilder {
        BoardBuilder::default()
    }

    pub fn start_position() -> Self {
        let white = [
            (Piece::Pawn, bitboards::WHITE_START_PAWNS),
            (Piece::Rook, bitboards::WHITE_START_ROOKS),
            (Piece::Knight, bitboards::WHITE_START_KNIGHTS),
            (Piece::Bishop, bitboards::WHITE_START_BISHOPS),
            (Piece::Queen, bitboards::WHITE_START_QUEENS),
            (Piece::King, bitboards::WHITE_START_KINGS)
        ].iter().cloned().collect();

        let black = [
            (Piece::Pawn, bitboards::BLACK_START_PAWNS),
            (Piece::Rook, bitboards::BLACK_START_ROOKS),
            (Piece::Knight, bitboards::BLACK_START_KNIGHTS),
            (Piece::Bishop, bitboards::BLACK_START_BISHOPS),
            (Piece::Queen, bitboards::BLACK_START_QUEENS),
            (Piece::King, bitboards::BLACK_START_KINGS)
        ].iter().cloned().collect();

        Self::builder()
            .white_pieces(white)
            .black_pieces(black)
            .player_turn(Player::White)
            .en_passant(Option::None)
            .white_castle_rights(CastleRights::Both)
            .black_castle_rights(CastleRights::Both)
            .draw_half_turns(0)
            .full_turns(0)
            .build().expect("Board start position builder panicked")
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::builder()
            .white_pieces(HashMap::new())
            .black_pieces(HashMap::new())
            .player_turn(Player::White)
            .en_passant(Option::None)
            .white_castle_rights(CastleRights::None)
            .black_castle_rights(CastleRights::None)
            .draw_half_turns(0)
            .full_turns(0)
            .build().expect("Board default builder panicked")
    }
}

//---------------------------------------------------------------------------
// Display
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Gets the character for a given piece.
        fn piece_char(piece: Piece, player: Player) -> char {
            let val = match player {
                Player::White => 0x2654,
                Player::Black => 0x265A
            } + match piece {
                Piece::King => 0,
                Piece::Queen => 1,
                Piece::Rook => 2,
                Piece::Bishop => 3,
                Piece::Knight => 4,
                Piece::Pawn => 5
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
                    .filter(|sq| *sq == square)
                    .map(|_| 'e')
                    .or_else(|| {
                        let mut res: Option<(Piece, Player)> = None;
                        for (&key, value) in self.white_pieces.iter() {
                            if value.is_square_set(square) {
                                res = Option::Some((key, Player::White))
                            }
                        };
                        for (&key, value) in self.black_pieces.iter() {
                            if value.is_square_set(square) {
                                res = Option::Some((key, Player::Black))
                            }
                        }
                        res.map(|(piece, player)| piece_char(piece, player))
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
