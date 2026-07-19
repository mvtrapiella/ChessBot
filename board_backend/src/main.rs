// src/main.rs
mod board;

use board::state::Board;
use board::types::{Color, NO_SQUARE};

fn main() {
    println!("Initializing the chess bot Atlas...");

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
    };

    board.initialize_board();
    board.update_bitboards();
    board.print_board();
}