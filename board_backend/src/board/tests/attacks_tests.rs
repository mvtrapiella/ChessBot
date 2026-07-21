use crate::board::Color;
use crate::board::types::{WHITE_KING, BLACK_ROOK, WHITE_PAWN, BLACK_KNIGHT};
use super::test_utils::{empty_board, place};

#[test]
fn rook_check_along_open_rank() {
    let mut board = empty_board();
    place(&mut board, 4, WHITE_KING);  // e1
    place(&mut board, 0, BLACK_ROOK);  // a1
    board.update_bitboards();

    assert!(board.is_square_attacked(4, Color::Black));

    // Block the rank between the rook and the king.
    place(&mut board, 2, WHITE_PAWN);  // c1
    board.update_bitboards();

    assert!(!board.is_square_attacked(4, Color::Black));
}

#[test]
fn knight_attack_is_detected() {
    let mut board = empty_board();
    place(&mut board, 4, WHITE_KING);    // e1
    place(&mut board, 19, BLACK_KNIGHT); // d3
    board.update_bitboards();

    assert!(board.is_square_attacked(4, Color::Black));
}
