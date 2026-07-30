// src/main.rs
mod board;

use std::collections::HashMap;
use std::io::stdin;

use board::state::Board;
use board::types::{Color, NO_SQUARE};

use crate::board::position::{self, Position};

fn ask_for_depth() -> u32 {
    loop {
        println!("Enter the search depth (a positive integer):");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Error reading the line");

        match input.trim().parse::<u32>() {
            Ok(depth) if depth > 0 => return depth,
            _ => println!("Depth must be a positive integer"),
        }
    }
}

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
        zobrian_hash: 0,
    };

    board.initialize_board();
    board.update_bitboards();

    let mut position = Position { board, history: Vec::new(), transposition_table: HashMap::new() };
    
    let depth = ask_for_depth();
    position.game_play(depth);
}