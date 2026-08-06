use board_backend::board::position::{GameStatus, Position};
use board_backend::board::types::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, EMPTY,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};

pub fn piece_letter(piece: u8) -> &'static str {
    match piece {
        WHITE_KNIGHT | BLACK_KNIGHT => "N",
        WHITE_BISHOP | BLACK_BISHOP => "B",
        WHITE_ROOK | BLACK_ROOK => "R",
        WHITE_QUEEN | BLACK_QUEEN => "Q",
        WHITE_KING | BLACK_KING => "K",
        _ => "",
    }
}

// Simplified algebraic notation: piece letter, captures, castling, promotion,
// and check/checkmate. Doesn't disambiguate when two identical pieces could
// reach the same square (rare, and not worth the extra complexity for a
// casual UI) -- e.g. two knights that could both legally land on the same
// square will both render as plain "Nf3" rather than "N1f3"/"N5f3".
pub fn describe_move(
    squares_before: [u8; 64],
    origin: u8,
    destination: u8,
    promotion_letter: Option<char>,
    position_after: &Position,
) -> String {
    let moved_piece = squares_before[origin as usize];

    let is_king = matches!(moved_piece, WHITE_KING | BLACK_KING);
    if is_king && (origin as i16 - destination as i16).abs() == 2 {
        let is_kingside = destination == 6 || destination == 62;
        return if is_kingside { "O-O".to_string() } else { "O-O-O".to_string() };
    }

    let is_pawn = matches!(moved_piece, WHITE_PAWN | BLACK_PAWN);
    let origin_file = origin % 8;
    let destination_file = destination % 8;
    let is_capture =
        squares_before[destination as usize] != EMPTY || (is_pawn && origin_file != destination_file);

    let letter = piece_letter(moved_piece);
    let capture_prefix = if is_pawn && is_capture {
        format!("{}x", (b'a' + origin_file) as char)
    } else if is_capture {
        "x".to_string()
    } else {
        String::new()
    };

    let destination_str = Position::square_to_str(destination);
    let promotion_str = promotion_letter.map(|c| format!("={}", c)).unwrap_or_default();

    let opponent = position_after.board.side_to_move;
    let suffix = if position_after.board.is_in_check(opponent) {
        match position_after.game_status() {
            GameStatus::WhiteWon | GameStatus::BlackWon => "#",
            _ => "+",
        }
    } else {
        ""
    };

    format!("{}{}{}{}{}", letter, capture_prefix, destination_str, promotion_str, suffix)
}
