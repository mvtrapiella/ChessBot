use crate::board::state::Board;
use crate::board::types::{Color, EMPTY, NO_SQUARE};

pub fn empty_board() -> Board {
    Board {
        piece_bitboards: [0; 12],
        white_pieces: 0,
        black_pieces: 0,
        all_pieces: 0,
        squares: [EMPTY; 64],
        side_to_move: Color::White,
        castling_rights: 0,
        en_passant_square: NO_SQUARE,
        halfmove_clock: 0,
    }
}

pub fn place(board: &mut Board, square: u8, piece: u8) {
    board.squares[square as usize] = piece;
}

// Cross-checks incrementally-updated bitboards against a full rebuild from squares[],
// so a desync (as opposed to a squares[] mistake, which the other assertions already catch)
// gets caught explicitly.
pub fn assert_bitboards_consistent(board: &Board) {
    let mut rebuilt = *board;
    rebuilt.update_bitboards();

    assert_eq!(board.piece_bitboards, rebuilt.piece_bitboards, "piece_bitboards inconsistent with squares[]");
    assert_eq!(board.white_pieces, rebuilt.white_pieces, "white_pieces inconsistent with squares[]");
    assert_eq!(board.black_pieces, rebuilt.black_pieces, "black_pieces inconsistent with squares[]");
    assert_eq!(board.all_pieces, rebuilt.all_pieces, "all_pieces inconsistent with squares[]");
}
