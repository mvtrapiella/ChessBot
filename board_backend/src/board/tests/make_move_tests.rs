use crate::board::position::Position;
use crate::board::state::Board;
use crate::board::types::{
    Move, EMPTY, WHITE_KING, WHITE_ROOK, WHITE_PAWN, WHITE_QUEEN, BLACK_ROOK, BLACK_PAWN,
};
use super::test_utils::{empty_board, place, assert_bitboards_consistent};

fn position_with(pieces: &[(u8, u8)]) -> Position {
    let mut board: Board = empty_board();
    for &(square, piece) in pieces {
        place(&mut board, square, piece);
    }
    board.update_bitboards();
    Position { board, history: Vec::new() }
}

#[test]
fn capture_keeps_all_bitboards_in_sync() {
    let mut pos = position_with(&[(4, WHITE_KING), (0, WHITE_ROOK), (7, BLACK_ROOK)]);
    pos.make_move(Move { origin: 0, destination: 7, promotion: None }); // a1 rook captures h1 rook

    assert_eq!(pos.board.squares[7], WHITE_ROOK);
    assert_eq!(pos.board.squares[0], EMPTY);
    assert_bitboards_consistent(&pos.board);
}

#[test]
fn promotion_updates_piece_bitboards_not_pawn() {
    let mut pos = position_with(&[(4, WHITE_KING), (52, WHITE_PAWN)]); // e7
    pos.make_move(Move { origin: 52, destination: 60, promotion: Some(WHITE_QUEEN) }); // e8=Q

    assert_eq!(pos.board.squares[60], WHITE_QUEEN);
    assert_bitboards_consistent(&pos.board);
}

#[test]
fn castling_moves_rook_and_stays_consistent() {
    let mut pos = position_with(&[(4, WHITE_KING), (7, WHITE_ROOK)]);
    pos.board.castling_rights = 0b0001; // white short castle only
    pos.make_move(Move { origin: 4, destination: 6, promotion: None });

    assert_eq!(pos.board.squares[6], WHITE_KING);
    assert_eq!(pos.board.squares[5], WHITE_ROOK);
    assert_eq!(pos.board.squares[7], EMPTY);
    assert_eq!(pos.board.castling_rights, 0);
    assert_bitboards_consistent(&pos.board);
}

#[test]
fn en_passant_removes_pawn_from_correct_square() {
    let mut pos = position_with(&[(4, WHITE_KING), (36, WHITE_PAWN), (37, BLACK_PAWN)]); // e5, f5
    pos.board.en_passant_square = 45; // f6

    pos.make_move(Move { origin: 36, destination: 45, promotion: None });

    assert_eq!(pos.board.squares[45], WHITE_PAWN);
    assert_eq!(pos.board.squares[37], EMPTY); // captured black pawn removed from f5, not f6
    assert_bitboards_consistent(&pos.board);
}
