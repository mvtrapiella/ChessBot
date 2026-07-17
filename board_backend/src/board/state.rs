// src/board/state.rs
use super::types::Color::{self, White, Black};
use super::types::{EMPTY, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING, BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING};

pub struct Board{
    /* Bitboards */
    // Each bitboard occupies 8 bytes (64 bits) so there is no padding and the register space is used efficientely
    pub piece_bitboards: [u64; 12], // 12 bitboards: 6 for each type of white pieces and the other 6 for the black ones
    pub white_pieces: u64, // position of all the white pieces -> union of all the white piece bitboards
    pub black_pieces: u64, // position of all the black pieces -> union of all the black piece bitboards
    pub all_pieces: u64,  // position of all the pieces on the board -> union of white and black bitboards

    // The full board occupies 64 bytes which is divided in 8 registers of 8 bytes with no padding -> perfect memory usage
    pub squares: [u8; 64], // board representation. Each of the 64 cells contains a number that represnt 
    // the piece on it (1-6 white pieces, 7-12 black pieces) and 0 if it empty. It coincides with the L1 cache of 64 bytes so there is no risk of a cache miss when reading the board

    // u8 -> 8 bits -> 1 byte
    pub side_to_move: Color, // 0 white turn, 1 black turn -> change by applying XOR
    // u8 -> 8 bits -> 1 byte
    pub castling_rights: u8,  // 0000 1111 each of the first 4 bits represent a posibility. If is one the castle is allowed,
    // so if the value is 15 all the castles are allowed, if 14 (0000 1110) all except the white short castle, if 12 (0000 1100) only the blacks can castle ...
    // u8 -> 8 bits -> 1 byte  
    pub en_passant_square: u8, // Stores the cell to which the rival pawn could move (eat) in case it is on the correct position for doing on pasant
    // u8 -> 8 bits -> 1 byte  
    pub halfmove_clock: u8,  // Counter that if it reaches 100 (50 moves each player) without capturing a piece it is automatically a drawn 
    // Total is 4 bytes -> which makes that the register not generate padding -> perfect memory usage
}

impl Board {
    /// Change the player turn by applying a XOR operation at bit level 
    pub fn switch_turn(&mut self) {
        let current_color_numeric = self.side_to_move as u8;
        
        // XOR: 0 ^ 1 = 1 (Black), 1 ^ 1 = 0 (White)
        let next_color_numeric = current_color_numeric ^ 1;
        
        self.side_to_move = unsafe { std::mem::transmute(next_color_numeric) };
    }

    pub fn initialize_board(&mut self) {
        // --- WHITE PIECES ---
        self.squares[0] = WHITE_ROOK;
        self.squares[1] = WHITE_KNIGHT;
        self.squares[2] = WHITE_BISHOP;
        self.squares[3] = WHITE_QUEEN; 
        self.squares[4] = WHITE_KING;  
        self.squares[5] = WHITE_BISHOP;
        self.squares[6] = WHITE_KNIGHT;
        self.squares[7] = WHITE_ROOK;

        // Pawn file (second row)
        for i in 8..16 {
            self.squares[i] = WHITE_PAWN;
        }

        // --- BLACK PIECES ---
        self.squares[56] = BLACK_ROOK;
        self.squares[57] = BLACK_KNIGHT;
        self.squares[58] = BLACK_BISHOP;
        self.squares[59] = BLACK_QUEEN; 
        self.squares[60] = BLACK_KING;  
        self.squares[61] = BLACK_BISHOP;
        self.squares[62] = BLACK_KNIGHT;
        self.squares[63] = BLACK_ROOK;

        // Pawn file (seventh row)
        for i in 48..56 {
            self.squares[i] = BLACK_PAWN;
        }
    }

    pub fn update_bitboards(&mut self) {
        // 1. Clean all the bitboards
        self.piece_bitboards = [0; 12];
        self.white_pieces = 0;
        self.black_pieces = 0;
        self.all_pieces = 0;

        // 2. We traverse the full 64 board
        for square in 0..64 {
            let piece = self.squares[square];

            // If the cell is empty we skip to the next one
            if piece == EMPTY {
                continue;
            }

            // We create the mask for bit of the board in which is the piece
            let bitmask: u64 = 1_u64 << square;

            // We calculate the index of the piece on the array of bitboard. Because the arrays starts in the index 0 and our pieces go from 1-16 we have to subtract one.
            // We have to cast to usize because we cannot index using u8 so we transform it to usize (the natural unit of indexing) so the compiler do not crash
            let bitboard_index = (piece - 1) as usize;

            // We activate the bit o the bitboard representing the board for each piece type. For that, we apply the OR operation
            self.piece_bitboards[bitboard_index] |= bitmask;

            // We activate the bits also in the corresponing white and black bitboards
            // Is White
            if piece <= 6 {
                self.white_pieces |= bitmask;
            } 
            // Is Black
            else {
                self.black_pieces |= bitmask;
            }
        }

        // We do the same for the bitboard representing all the pieces
        self.all_pieces = self.white_pieces | self.black_pieces;
    }

    pub fn print_board(&self) {
        println!("\n  +-------------------------------+");

        // We start by the end so in the console the whites appears under the black ones (the blacks are the first printed)
        for rank in (0..8).rev() {
            // We print the file number
            print!("{} | ", rank + 1);

            for file in 0..8 {
                let square_index = 8 * rank + file;
                let piece = self.squares[square_index];

                let piece_char = match piece {
                    1 => "♙", // White Pawn
                    2 => "♖", // White Rook
                    3 => "♘", // White Knight
                    4 => "♗", // White Bishop
                    5 => "♕", // White Queen
                    6 => "♔", // White King
                    7 => "♟", // Black Pawn
                    8 => "♜", // Black Rook
                    9 => "♞", // Black Knight
                    10 => "♝", // Black Bishop
                    11 => "♛", // Black Queen
                    12 => "♚", // Black King
                    _ => ".",  // Empty
                };
                
                print!("{} | ", piece_char);
            }

            println!("");
        }

        println!("  +-------------------------------+");
        println!("    a   b   c   d   e   f   g   h\n");

        let side = match self.side_to_move {
            White => "White",
            Black => "Black",
        };

        println!("{} player to move", side);        
    }
}