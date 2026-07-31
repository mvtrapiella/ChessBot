use crate::board::Color;
use crate::board::types::{WHITE_KING, WHITE_BISHOP, WHITE_ROOK, WHITE_PAWN, BLACK_ROOK, BLACK_PAWN};
use super::test_utils::{empty_board, place};

#[test]
fn pinned_bishop_has_no_legal_moves() {
    let mut board = empty_board();
    place(&mut board, 4, WHITE_KING);    // e1
    place(&mut board, 12, WHITE_BISHOP); // e2
    place(&mut board, 60, BLACK_ROOK);   // e8
    board.update_bitboards();

    assert!(!board.move_generator(12, WHITE_BISHOP).is_empty());
    assert!(board.legal_moves(12, WHITE_BISHOP).is_empty());
}

#[test]
fn en_passant_discovered_check_is_excluded() {
    let mut board = empty_board();
    place(&mut board, 32, WHITE_KING); // a5
    place(&mut board, 39, BLACK_ROOK); // h5
    place(&mut board, 36, WHITE_PAWN); // e5
    place(&mut board, 37, BLACK_PAWN); // f5
    board.en_passant_square = 45;      // f6
    board.update_bitboards();

    let legal = board.legal_moves(36, WHITE_PAWN);
    assert!(!legal.iter().any(|mv| mv.destination == 45));
}

#[test]
fn castling_through_attacked_square_is_illegal() {
    let mut board = empty_board();
    place(&mut board, 4, WHITE_KING);  // e1
    place(&mut board, 7, WHITE_ROOK);  // h1
    place(&mut board, 61, BLACK_ROOK); // f8, attacks the f1 transit square
    board.castling_rights = 1;         // white short castle only
    board.update_bitboards();

    assert!(board.is_square_attacked(5, Color::Black)); // f1 attacked
    let legal = board.legal_moves(4, WHITE_KING);
    assert!(!legal.iter().any(|mv| mv.destination == 6));
}

#[test]
fn castling_while_in_check_is_illegal() {
    let mut board = empty_board();
    place(&mut board, 4, WHITE_KING);  // e1
    place(&mut board, 7, WHITE_ROOK);  // h1
    place(&mut board, 60, BLACK_ROOK); // e8, checks the king directly
    board.castling_rights = 1;
    board.update_bitboards();

    let legal = board.legal_moves(4, WHITE_KING);
    assert!(!legal.iter().any(|mv| mv.destination == 6));
}
