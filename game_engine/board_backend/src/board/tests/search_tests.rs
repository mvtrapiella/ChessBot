use crate::board::position::Position;
use crate::board::negamax::SearchLimit;
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
    Position { board, history: Vec::new(), transposition_table: Vec::new(), position_history: Vec::new(), moves_counter: 0, search_path_hashes: Vec::new(), nodes: 0, deadline: None, search_aborted: false }
}

fn starting_position() -> Position {
    let mut board = Board {
        squares: [0; 64],
        piece_bitboards: [0; 12],
        white_pieces: 0,
        black_pieces: 0,
        all_pieces: 0,
        side_to_move: Color::White,
        castling_rights: 15,
        en_passant_square: crate::board::types::NO_SQUARE,
        halfmove_clock: 0,
        zobrian_hash: 0,
    };

    board.initialize_board();
    board.update_bitboards();

    Position { board, history: Vec::new(), transposition_table: Vec::new(), position_history: Vec::new(), moves_counter: 0, search_path_hashes: Vec::new(), nodes: 0, deadline: None, search_aborted: false }
}

fn play(moves: &[&str]) -> Position {
    let mut position = starting_position();
    for mv in moves {
        position.apply_move_str(mv).expect("test move should be legal");
    }
    position
}

#[test]
fn find_best_move_remaps_a_book_recommended_castle_to_a_legal_move() {
    // 1. e4 c5 2. Nf3 d6 3. Bb5 Nc6 -- gm2001.bin's top-weighted reply here is White
    // kingside castling, stored Polyglot-style as e1h1 (king "takes" its own rook).
    // Without the remap this would fail the legal-move check and silently fall through
    // to search instead.
    let mut pos = play(&["e2e4", "c7c5", "g1f3", "d7d6", "f1b5", "b8c6"]);

    let best = pos.find_best_move(SearchLimit::Depth(1));

    assert_eq!(best, Some(Move { origin: 4, destination: 6, promotion: None }));
}

#[test]
fn find_best_move_plays_the_book_move_from_the_starting_position() {
    // e2e4 is the highest-weighted reply to the starting position in the embedded
    // gm2001.bin book (independently confirmed via python-chess's own book reader) --
    // find_best_move should return it directly from the book, without needing to search
    // at all, regardless of how high a depth is requested.
    let mut pos = starting_position();

    let best = pos.find_best_move(SearchLimit::Depth(1));

    assert_eq!(best, Some(Move { origin: 12, destination: 28, promotion: None }));
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

    let best = pos.find_best_move(SearchLimit::Depth(1));

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
    assert_eq!(pos.find_best_move(SearchLimit::Depth(1)), None);

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

    let best = pos.find_best_move(SearchLimit::Depth(2));

    assert_eq!(best, Some(Move { origin: 27, destination: 63, promotion: None }));
}
