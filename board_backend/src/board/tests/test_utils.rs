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
