//! Parses FEN notation into game state.

use board::piece::Piece;
use board::player::Player;
use board::square::Square;
use rules::castle_rights::CastleRights;
use rules::game_state::GameState;
use rules::player_board::PlayerBoard;
use std::str::FromStr;

/// Parses a given FEN string.
///
/// A FEN record contains six fields. The separator between fields is a space. The fields are:
///
/// - Piece placement (from white's perspective). Each rank is described, starting with rank 8
///   and ending with rank 1; within each rank, the contents of each square are described from
///   file "a" through file "h". Following the Standard Algebraic Notation (SAN), each piece
///   is identified by a single letter taken from the standard English names
///   (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K").
///   White pieces are designated using upper-case letters ("PNBRQK") while black pieces
///   use lowercase ("pnbrqk"). Empty squares are noted using digits 1 through 8
///   (the number of empty squares), and "/" separates ranks.
/// - Active colour. "w" means White moves next, "b" means Black.
/// - Castling availability. If neither side can castle, this is "-". Otherwise, this
///   has one or more letters: "K" (White can castle kingside), "Q" (White can castle queenside),
///   "k" (Black can castle kingside), and/or "q" (Black can castle queenside).
/// - En passant target square in algebraic notation. If there's no en passant target square,
///   this is "-". If a pawn has just made a two-square move, this is the position "behind"
///   the pawn. This is recorded regardless of whether there is a pawn in position to make an
///   en passant capture.
/// - Halfmove clock: This is the number of halfmoves since the last capture or pawn advance.
///   This is used to determine if a draw can be claimed under the fifty-move rule.
/// - Fullmove number: The number of the full move. It starts at 1, and is incremented
///   after Black's move.
pub fn parse_fen(fen: &str) -> Result<GameState, String> {
    info!("Parsing FEN: '{}'", fen);
    let parts = fen.split(" ").collect::<Vec<_>>();

    if parts.len() != 6 {
        return Err(format!("FEN hasn't got exactly 6 required parts: {}", fen));
    }

    let pieces = parse_piece_placement(parts[0])?;
    let active = parse_active_color(parts[1])?;
    let castling = parse_castling_ability(parts[2])?;
    let en_passant = parse_en_passant(parts[3])?;
    let half_moves = parse_number(parts[4])?;
    let full_moves = parse_number(parts[5])?;

    Ok(GameState {
        white_board: pieces.white,
        black_board: pieces.black,
        player_turn: active,
        en_passant,
        white_castle_rights: castling.white,
        black_castle_rights: castling.black,
        draw_plies: half_moves,
        full_turns: full_moves - 1,
    })
}

// represents a result for each player
struct PlayerValues<T> {
    white: T,
    black: T,
}

// ---------------------------------------------------
// parses the piece placement part
// TODO this needs to be flipped??
fn parse_piece_placement(part: &str) -> Result<PlayerValues<PlayerBoard>, String> {
    // can literally step through the string and parse it as needed.
    let mut white = PlayerBoard::new();
    let mut black = PlayerBoard::new();
    let mut square_index: u8 = 0;
    let chars = part.chars();
    for ch in chars {
        // move to next rank
        if ch == '/' {
            if square_index % 8 != 0 {
                return Err(format!("Some rows aren't 8 pieces long"));
            }
            continue;
        }
        // skip empty squares
        if let Some(digit) = ch.to_digit(10) {
            square_index += digit as u8;
            continue;
        }
        let (player, piece) = parse_fen_piece(ch)?;
        // FEN uses the reverse order to our internal representation
        let square = Square::new(square_index).mirror_horizontal();
        match player {
            Player::White => white = white.with_piece(piece, white.piece(piece).set_square(square)),
            Player::Black => black = black.with_piece(piece, black.piece(piece).set_square(square)),
        }
        square_index += 1;
    }
    if square_index != 64 {
        return Err(format!("Piece sequence doesn't cover 64 squares: {}", part));
    }
    Ok(PlayerValues { white, black })
}

// parses the given FEN piece identifier
fn parse_fen_piece(ch: char) -> Result<(Player, Piece), String> {
    match ch {
        'P' => Ok((Player::White, Piece::Pawn)),
        'N' => Ok((Player::White, Piece::Knight)),
        'B' => Ok((Player::White, Piece::Bishop)),
        'R' => Ok((Player::White, Piece::Rook)),
        'Q' => Ok((Player::White, Piece::Queen)),
        'K' => Ok((Player::White, Piece::King)),
        'p' => Ok((Player::Black, Piece::Pawn)),
        'n' => Ok((Player::Black, Piece::Knight)),
        'b' => Ok((Player::Black, Piece::Bishop)),
        'r' => Ok((Player::Black, Piece::Rook)),
        'q' => Ok((Player::Black, Piece::Queen)),
        'k' => Ok((Player::Black, Piece::King)),
        _ => Err(format!("Invalid piece identifier: {}", ch)),
    }
}

// parses the active color part
fn parse_active_color(part: &str) -> Result<Player, String> {
    match part {
        "b" => Ok(Player::Black),
        "w" => Ok(Player::White),
        _ => Err(format!(
            "Unknown active color value: {}. Expected 'w' or 'b'",
            part
        )),
    }
}

// parses the castling ability part
fn parse_castling_ability(part: &str) -> Result<PlayerValues<CastleRights>, String> {
    if part == "-" {
        return Ok(PlayerValues {
            black: CastleRights::None,
            white: CastleRights::None,
        });
    }
    let mut white = CastleRights::None;
    let mut black = CastleRights::None;
    for char in part.chars() {
        match char {
            'k' => white = white.with_king_side(),
            'q' => white = white.with_queen_side(),
            'K' => black = black.with_king_side(),
            'Q' => black = black.with_queen_side(),
            _ => return Err(format!("Unknown castle ability character {}", char)),
        }
    }
    Ok(PlayerValues { black, white })
}

// parses the en passant square part
fn parse_en_passant(part: &str) -> Result<Option<Square>, String> {
    if part == "-" {
        return Ok(None);
    }
    Square::from_str(part).map(|sq| Some(sq))
}

// parses the half-move or full-move clock part
fn parse_number(part: &str) -> Result<u8, String> {
    u8::from_str(part).map_err(|er| er.to_string())
}
