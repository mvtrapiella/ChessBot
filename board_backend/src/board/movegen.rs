// src/board/movegen.rs
use super::state::Board;
use super::masks::king_masks::KING_ATTACKS;
use super::masks::knight_masks::KNIGHT_ATTACKS;
use super::masks::bishop_masks::{BISHOP_MAGICS, BISHOP_SHIFTS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_ATTACKS_TABLE};
use super::masks::rook_masks::{ROOK_MAGICS, ROOK_SHIFTS, ROOK_MASKS, ROOK_OFFSETS, ROOK_ATTACKS_TABLE};
use super::types::{
    Move, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, NO_SQUARE,
};

impl Board {
    pub fn move_generator(&self, origin: u8, piece: u8) -> Vec<Move> {
        let mut moves = Vec::new();

        match piece {
            WHITE_PAWN => self.generate_white_pawn_moves(origin, &mut moves),
            BLACK_PAWN => self.generate_black_pawn_moves(origin, &mut moves),
            WHITE_KNIGHT => self.generate_white_knight_moves(origin, &mut moves),
            BLACK_KNIGHT => self.generate_black_knight_moves(origin, &mut moves),
            WHITE_BISHOP => self.generate_white_bishop_moves(origin, &mut moves),
            BLACK_BISHOP => self.generate_black_bishop_moves(origin, &mut moves),
            WHITE_ROOK => self.generate_white_rook_moves(origin, &mut moves),
            BLACK_ROOK => self.generate_black_rook_moves(origin, &mut moves),
            WHITE_QUEEN => self.generate_white_queen_moves(origin, &mut moves),
            BLACK_QUEEN => self.generate_black_queen_moves(origin, &mut moves),
            WHITE_KING => self.generate_white_king_moves(origin, &mut moves),
            BLACK_KING => self.generate_black_king_moves(origin, &mut moves),
            _ => panic!("move_generator called with invalid piece byte: {}", piece),
        }

        moves
    }

    fn generate_white_king_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let mut valid_attacks = KING_ATTACKS[origin as usize] & !self.white_pieces;

        while valid_attacks != 0 {
            // Native function of rust to count the number of zeros at the right of the least one
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            // We eliminare the last one. Example -> 0100 1000 - 1 = 0100 0111 and if we apply 0100 0111 & 0100 1000 = 0100 0000
            // We have remove the least significant bit and make the LSB the next smaller bit
            valid_attacks &= valid_attacks - 1;
        }

        // Squares that must be empty: f1(5), g1(6)
        let short_castle_empty = 0x0000000000000060;
        // Squares that must be empty: b1(1), c1(2), d1(3)
        let long_castle_empty = 0x000000000000000E;

        if ((short_castle_empty & self.all_pieces) == 0) && ((1u8 & self.castling_rights) != 0) {
            // King
            moves.push(Move{origin: origin, destination: 6, promotion: None});
            // Rook
            moves.push(Move{origin: 7, destination: 5, promotion: None});
        }

        if ((long_castle_empty & self.all_pieces) == 0) && ((2u8 & self.castling_rights) != 0) {
            // King
            moves.push(Move{origin: origin, destination: 2, promotion: None});
            // Rook
            moves.push(Move{origin: 0, destination: 3, promotion: None});
        }
    }

    fn generate_black_king_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let mut valid_attacks = KING_ATTACKS[origin as usize] & !self.black_pieces;

        while valid_attacks != 0 {
            // Native function of rust to count the number of zeros at the right of the least one
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            // We eliminare the last one. Example -> 0100 1000 - 1 = 0100 0111 and if we apply 0100 0111 & 0100 1000 = 0100 0000
            // We have remove the least significant bit and make the LSB the next smaller bit
            valid_attacks &= valid_attacks - 1;
        }

        // Squares that must be empty: f8(61), g8(62)
        let short_castle_empty = 0x6000000000000000;
        // Squares that must be empty: b8(57), c8(58), d8(59)
        let long_castle_empty = 0x0E00000000000000;

        if ((short_castle_empty & self.all_pieces) == 0) && ((4u8 & self.castling_rights) != 0) {
            // King
            moves.push(Move{origin: origin, destination: 62, promotion: None});
            // Rook
            moves.push(Move{origin: 63, destination: 61, promotion: None});
        }

        if ((long_castle_empty & self.all_pieces) == 0) && ((8u8 & self.castling_rights) != 0) {
            // King
            moves.push(Move{origin: origin, destination: 58, promotion: None});
            // Rook
            moves.push(Move{origin: 56, destination: 59, promotion: None});
        }
    }

    fn generate_white_queen_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let square = origin as usize;

        // Rook move
        let rook_blockers = self.all_pieces & ROOK_MASKS[square];
        let rook_magic = ROOK_MAGICS[square];
        let rook_shift = ROOK_SHIFTS[square];
        let rook_offset = ROOK_OFFSETS[square];

        // Bishop move
        let bishop_blockers = self.all_pieces & BISHOP_MASKS[square];
        let bishop_magic = BISHOP_MAGICS[square];
        let bishop_shift = BISHOP_SHIFTS[square];
        let bishop_offset = BISHOP_OFFSETS[square];

        let rook_hash = (rook_blockers.wrapping_mul(rook_magic) >> rook_shift) as usize;

        let mut valid_attacks = ROOK_ATTACKS_TABLE[rook_hash + rook_offset];

        let bishop_hash = (bishop_blockers.wrapping_mul(bishop_magic) >> bishop_shift) as usize;

        valid_attacks |= BISHOP_ATTACKS_TABLE[bishop_hash + bishop_offset];

        valid_attacks &= !self.white_pieces;

        while valid_attacks != 0 {
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_black_queen_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let square = origin as usize;

        // Rook move
        let rook_blockers = self.all_pieces & ROOK_MASKS[square];
        let rook_magic = ROOK_MAGICS[square];
        let rook_shift = ROOK_SHIFTS[square];
        let rook_offset = ROOK_OFFSETS[square];

        // Bishop move
        let bishop_blockers = self.all_pieces & BISHOP_MASKS[square];
        let bishop_magic = BISHOP_MAGICS[square];
        let bishop_shift = BISHOP_SHIFTS[square];
        let bishop_offset = BISHOP_OFFSETS[square];

        let rook_hash = (rook_blockers.wrapping_mul(rook_magic) >> rook_shift) as usize;

        let mut valid_attacks = ROOK_ATTACKS_TABLE[rook_hash + rook_offset];

        let bishop_hash = (bishop_blockers.wrapping_mul(bishop_magic) >> bishop_shift) as usize;

        valid_attacks |= BISHOP_ATTACKS_TABLE[bishop_hash + bishop_offset];

        valid_attacks &= !self.black_pieces;

        while valid_attacks != 0 {
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_white_rook_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let square = origin as usize;
        let blockers = self.all_pieces & ROOK_MASKS[square];
        let magic = ROOK_MAGICS[square];
        let shift = ROOK_SHIFTS[square];
        let offset = ROOK_OFFSETS[square];
        
        let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

        let mut valid_attacks = ROOK_ATTACKS_TABLE[hash + offset];

        valid_attacks &= !self.white_pieces;

        while valid_attacks != 0 {
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_black_rook_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let square = origin as usize;
        let blockers = self.all_pieces & ROOK_MASKS[square];
        let magic = ROOK_MAGICS[square];
        let shift = ROOK_SHIFTS[square];
        let offset = ROOK_OFFSETS[square];
        
        let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

        let mut valid_attacks = ROOK_ATTACKS_TABLE[hash + offset];

        valid_attacks &= !self.black_pieces;

        while valid_attacks != 0 {
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_white_bishop_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let square = origin as usize;
        let blockers = self.all_pieces & BISHOP_MASKS[square];
        let magic = BISHOP_MAGICS[square];
        let shift = BISHOP_SHIFTS[square];
        let offset = BISHOP_OFFSETS[square];
        
        let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

        let mut valid_attacks = BISHOP_ATTACKS_TABLE[hash + offset];

        valid_attacks &= !self.white_pieces;

        while valid_attacks != 0 {
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_black_bishop_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let square = origin as usize;
        let blockers = self.all_pieces & BISHOP_MASKS[square];
        let magic = BISHOP_MAGICS[square];
        let shift = BISHOP_SHIFTS[square];
        let offset = BISHOP_OFFSETS[square];
        
        let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

        let mut valid_attacks = BISHOP_ATTACKS_TABLE[hash + offset];

        valid_attacks &= !self.black_pieces;

        while valid_attacks != 0 {
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_white_pawn_moves(&self, origin: u8, mut moves: &mut Vec<Move>){
        let column = origin % 8;

        // First move of the white pawn. It can be moved to squares forward
        if origin >= 8 && origin <= 15
            && (self.all_pieces & (1_u64 << (origin + 8))) == 0 
            && (self.all_pieces & (1_u64 << (origin + 16))) == 0 
        {
            moves.push(Move { origin, destination: origin + 16, promotion: None });
        }

        // Movement without promotion
        if origin < 48 {
            // One square forward
            if (self.all_pieces & (1_u64 << (origin + 8))) == 0 {
                moves.push(Move { origin, destination: origin + 8, promotion: None });
            }

            // Capture only right (A column pawns)
            if column == 0 {
                if (self.black_pieces & (1_u64 << (origin + 9))) != 0 {
                    moves.push(Move { origin, destination: origin + 9, promotion: None });
                }
                // En passant check safe from NO_SQUARE (64)
                if self.en_passant_square != NO_SQUARE && self.en_passant_square == origin + 9 {
                    moves.push(Move { origin, destination: origin + 9, promotion: None });
                }
            } 
            // Capture only left (H column pawns)
            else if column == 7 {
                if (self.black_pieces & (1_u64 << (origin + 7))) != 0 {
                    moves.push(Move { origin, destination: origin + 7, promotion: None });
                }
                // En passant check safe from NO_SQUARE (64)
                if self.en_passant_square != NO_SQUARE && self.en_passant_square == origin + 7 {
                    moves.push(Move { origin, destination: origin + 7, promotion: None });
                }
            } 
            // Capture at both directions (The rest of the pawns B-G)
            else {
                if (self.black_pieces & (1_u64 << (origin + 7))) != 0 {
                    moves.push(Move { origin, destination: origin + 7, promotion: None });
                }
                if (self.black_pieces & (1_u64 << (origin + 9))) != 0 {
                    moves.push(Move { origin, destination: origin + 9, promotion: None });
                }
                // En passant check safe from NO_SQUARE (64)
                if self.en_passant_square != NO_SQUARE {
                    if self.en_passant_square == origin + 7 {
                        moves.push(Move { origin, destination: origin + 7, promotion: None });
                    }
                    if self.en_passant_square == origin + 9 {
                        moves.push(Move { origin, destination: origin + 9, promotion: None });
                    }
                }
            }
        }
        // Movement with promotion
        else {
            // Move forward
            if (self.all_pieces & (1_u64 << (origin + 8))) == 0 {
                self.push_white_promotions(&mut moves, origin, origin + 8);
            }

            // Pawn on A column
            if column == 0 {
                // Capture to the right
                if (self.black_pieces & (1_u64 << (origin + 9))) != 0 {
                    self.push_white_promotions(&mut moves, origin, origin + 9);
                }
            } 
            // Pawn on H column
            else if column == 7 {
                if (self.black_pieces & (1_u64 << (origin + 7))) != 0 {
                    self.push_white_promotions(&mut moves, origin, origin + 7);
                }
            } 
            // Rest of the pawns
            else {
                if (self.black_pieces & (1_u64 << (origin + 7))) != 0 {
                    self.push_white_promotions(&mut moves, origin, origin + 7);
                }
                if (self.black_pieces & (1_u64 << (origin + 9))) != 0 {
                    self.push_white_promotions(&mut moves, origin, origin + 9);
                }
            }
        }
    }

    fn generate_white_knight_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let mut valid_attacks = KNIGHT_ATTACKS[origin as usize] & !self.white_pieces;

        while valid_attacks != 0 {
            // Native function of rust to count the number of zeros at the right of the least one
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            // We eliminare the last one. Example -> 0100 1000 - 1 = 0100 0111 and if we apply 0100 0111 & 0100 1000 = 0100 0000
            // We have remove the least significant bit and make the LSB the next smaller bit
            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_black_knight_moves(&self, origin: u8, moves: &mut Vec<Move>){
        let mut valid_attacks = KNIGHT_ATTACKS[origin as usize] & !self.black_pieces;

        while valid_attacks != 0 {
            // Native function of rust to count the number of zeros at the right of the least one
            let destination = valid_attacks.trailing_zeros() as u8;

            moves.push(Move{origin: origin, destination: destination, promotion: None});

            // We eliminare the last one. Example -> 0100 1000 - 1 = 0100 0111 and if we apply 0100 0111 & 0100 1000 = 0100 0000
            // We have remove the least significant bit and make the LSB the next smaller bit
            valid_attacks &= valid_attacks - 1;
        }
    }

    fn generate_black_pawn_moves(&self, origin: u8, mut moves: &mut Vec<Move>){
        let column = origin % 8;

        // First move of the black pawn. It can be moved two squares forward
        if origin >= 48 && origin <= 55
            && (self.all_pieces & (1_u64 << (origin - 8))) == 0 
            && (self.all_pieces & (1_u64 << (origin - 16))) == 0 
        {
            moves.push(Move { origin, destination: origin - 16, promotion: None });
        }

        // Movement without promotion
        if origin > 15 {
            // One square forward
            if (self.all_pieces & (1_u64 << (origin - 8))) == 0 {
                moves.push(Move { origin, destination: origin - 8, promotion: None });
            }

            // Capture only right (A column pawns)
            if column == 0 {
                if (self.white_pieces & (1_u64 << (origin - 7))) != 0 {
                    moves.push(Move { origin, destination: origin - 7, promotion: None });
                }
                // En passant check safe from NO_SQUARE (64)
                if self.en_passant_square != NO_SQUARE && self.en_passant_square == origin - 7 {
                    moves.push(Move { origin, destination: origin - 7, promotion: None });
                }
            } 
            // Capture only left (H column pawns)
            else if column == 7 {
                if (self.white_pieces & (1_u64 << (origin - 9))) != 0 {
                    moves.push(Move { origin, destination: origin - 9, promotion: None });
                }
                // En passant check safe from NO_SQUARE (64)
                if self.en_passant_square != NO_SQUARE && self.en_passant_square == origin - 9 {
                    moves.push(Move { origin, destination: origin - 9, promotion: None });
                }
            } 
            // Capture at both directions (The rest of the pawns B-G)
            else {
                if (self.white_pieces & (1_u64 << (origin - 7))) != 0 {
                    moves.push(Move { origin, destination: origin - 7, promotion: None });
                }
                if (self.white_pieces & (1_u64 << (origin - 9))) != 0 {
                    moves.push(Move { origin, destination: origin - 9, promotion: None });
                }
                // En passant check safe from NO_SQUARE (64)
                if self.en_passant_square != NO_SQUARE {
                    if self.en_passant_square == origin - 7 {
                        moves.push(Move { origin, destination: origin - 7, promotion: None });
                    }
                    if self.en_passant_square == origin - 9 {
                        moves.push(Move { origin, destination: origin - 9, promotion: None });
                    }
                }
            }
        }
        // Movement with promotion
        else {
            // Move forward
            if (self.all_pieces & (1_u64 << (origin - 8))) == 0 {
                self.push_black_promotions(&mut moves, origin, origin - 8);
            }

            // Pawn on A column
            if column == 0 {
                // Capture to the right
                if (self.white_pieces & (1_u64 << (origin - 7))) != 0 {
                    self.push_black_promotions(&mut moves, origin, origin - 7);
                }
            } 
            // Pawn on H column
            else if column == 7 {
                // Capture to the left
                if (self.white_pieces & (1_u64 << (origin - 9))) != 0 {
                    self.push_black_promotions(&mut moves, origin, origin - 9);
                }
            } 
            // Rest of the pawns
            else {
                if (self.white_pieces & (1_u64 << (origin - 7))) != 0 {
                    self.push_black_promotions(&mut moves, origin, origin - 7);
                }
                if (self.white_pieces & (1_u64 << (origin - 9))) != 0 {
                    self.push_black_promotions(&mut moves, origin, origin - 9);
                }
            }
        }
    }

    fn push_white_promotions(&self, moves: &mut Vec<Move>, origin: u8, destination: u8) {
        // Promote to a bishop
        moves.push(Move { origin, destination, promotion: Some(WHITE_BISHOP) });
        // Promote to a knight
        moves.push(Move { origin, destination, promotion: Some(WHITE_KNIGHT) });
        // Promote to a rook
        moves.push(Move { origin, destination, promotion: Some(WHITE_ROOK) });
        // Promote to a queen
        moves.push(Move { origin, destination, promotion: Some(WHITE_QUEEN) });
    }

    fn push_black_promotions(&self, moves: &mut Vec<Move>, origin: u8, destination: u8) {
        // Promote to a bishop
        moves.push(Move { origin, destination, promotion: Some(BLACK_BISHOP) });
        // Promote to a knight
        moves.push(Move { origin, destination, promotion: Some(BLACK_KNIGHT) });
        // Promote to a rook
        moves.push(Move { origin, destination, promotion: Some(BLACK_ROOK) });
        // Promote to a queen
        moves.push(Move { origin, destination, promotion: Some(BLACK_QUEEN) });
    }
}