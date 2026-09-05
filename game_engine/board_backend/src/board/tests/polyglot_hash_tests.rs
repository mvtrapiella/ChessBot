use crate::board::polyglot_hash::polyglot_hash;
use crate::board::position::{Position, TT_SIZE};
use crate::board::state::Board;
use crate::board::types::{Color, NO_SQUARE};

fn starting_position() -> Position {
    let mut board = Board {
        squares: [0; 64],
        piece_bitboards: [0; 12],
        white_pieces: 0,
        black_pieces: 0,
        all_pieces: 0,
        side_to_move: Color::White,
        castling_rights: 15,
        en_passant_square: NO_SQUARE,
        halfmove_clock: 0,
        zobrian_hash: 0,
    };

    board.initialize_board();
    board.update_bitboards();

    Position {
        board,
        history: Vec::new(),
        transposition_table: vec![None; TT_SIZE],
        position_history: Vec::new(),
        moves_counter: 0,
        search_path_hashes: Vec::new(),
        nodes: 0,
        deadline: None,
        search_aborted: false,
    }
}

fn play(moves: &[&str]) -> Position {
    let mut position = starting_position();
    for mv in moves {
        position.apply_move_str(mv).expect("test move should be legal");
    }
    position
}

// Every expected value below was independently computed with python-chess's
// chess.polyglot.zobrist_hash -- the same trusted source the RANDOM64 table itself
// came from -- not derived from this engine's own code.

#[test]
fn matches_python_chess_for_the_starting_position() {
    let position = starting_position();
    assert_eq!(polyglot_hash(&position.board), 0x463B96181691FC9C);
}

#[test]
fn matches_python_chess_when_en_passant_square_exists_but_is_not_capturable() {
    // 1. e4 sets en_passant_square, but no black pawn is adjacent yet -- this must NOT
    // be hashed in, unlike the naive "en_passant_square.is_some()" check.
    let position = play(&["e2e4"]);
    assert_eq!(polyglot_hash(&position.board), 0x823C9B50FD114196);
}

#[test]
fn matches_python_chess_when_en_passant_is_actually_capturable() {
    // 1. e4 Nf6 2. e5 d5 -- White's e5 pawn can actually capture en passant on d6.
    let position = play(&["e2e4", "g8f6", "e4e5", "d7d5"]);
    assert_eq!(polyglot_hash(&position.board), 0x2158459FF499F8E3);
}

#[test]
fn matches_python_chess_after_castling_rights_are_lost() {
    let position = play(&[
        "e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6", "e1g1",
    ]);
    assert_eq!(polyglot_hash(&position.board), 0x3EE55CE7EEC931BE);
}
