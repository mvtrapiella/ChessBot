use std::collections::HashMap;
use crate::board::position::Position;
use crate::board::state::Board;
use crate::board::types::{
    Move, Color, WHITE_KING, WHITE_ROOK, WHITE_QUEEN, WHITE_PAWN, BLACK_KING, BLACK_ROOK,
};
use super::test_utils::{empty_board, place};

fn position_with(pieces: &[(u8, u8)], side_to_move: Color) -> Position {
    let mut board: Board = empty_board();
    for &(square, piece) in pieces {
        place(&mut board, square, piece);
    }
    board.side_to_move = side_to_move;
    board.update_bitboards();
    Position { board, history: Vec::new(), transposition_table: HashMap::new(), position_history: Vec::new(), moves_counter: 0, search_path_hashes: Vec::new() }
}

#[test]
fn finds_mate_in_one() {
    // White king a3, rooks a7 + b1, black king h8, white to move.
    // Rb1-b8 checks along the 8th rank; the a7 rook already covers g7/h7,
    // and the b8 rook (after moving) covers g8 -- no escape, no mate in one.
    let mut pos = position_with(
        &[(16, WHITE_KING), (48, WHITE_ROOK), (1, WHITE_ROOK), (63, BLACK_KING)],
        Color::White,
    );

    let best = pos.find_best_move(1);

    assert_eq!(best, Some(Move { origin: 1, destination: 57, promotion: None }));
}

#[test]
fn recognizes_stalemate_as_a_draw() {
    // White king c7, white queen b6, black king a8, black to move.
    // a7/b7/b8 are all covered by the queen, a8 itself isn't attacked: stalemate.
    let mut pos = position_with(
        &[(50, WHITE_KING), (41, WHITE_QUEEN), (56, BLACK_KING)],
        Color::Black,
    );

    assert!(pos.board.all_legal_moves().is_empty());
    assert!(!pos.board.is_in_check(Color::Black));
    assert_eq!(pos.find_best_move(1), None);

    let score = pos.negamax(1, 0, -10_000_000, 10_000_000);
    assert_eq!(score, 0);
}

#[test]
fn prefers_a_free_capture_over_a_quiet_move() {
    // White king e1, white queen d4, white pawn a2, black king a8, black rook h8
    // (undefended). d4-h8 is a clear diagonal (Qxh8), while d4 shares no line with
    // a8 at all -- unlike e.g. a queen on h1, which would have a clear diagonal
    // straight to a8 and could pseudo-legally "capture" the black king itself.
    let mut pos = position_with(
        &[(4, WHITE_KING), (27, WHITE_QUEEN), (8, WHITE_PAWN), (56, BLACK_KING), (63, BLACK_ROOK)],
        Color::White,
    );

    let best = pos.find_best_move(2);

    assert_eq!(best, Some(Move { origin: 27, destination: 63, promotion: None }));
}
