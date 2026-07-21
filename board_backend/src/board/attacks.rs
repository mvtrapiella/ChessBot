use crate::board::{Board, Color};
use super::masks::king_masks::KING_ATTACKS;
use super::masks::knight_masks::KNIGHT_ATTACKS;
use super::masks::bishop_masks::{BISHOP_MAGICS, BISHOP_SHIFTS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_ATTACKS_TABLE};
use super::masks::rook_masks::{ROOK_MAGICS, ROOK_SHIFTS, ROOK_MASKS, ROOK_OFFSETS, ROOK_ATTACKS_TABLE};
use super::types::{
    Move, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, NO_SQUARE,
};

impl Board{

    pub fn is_in_check(&self, player: Color) -> bool{
        let mut king_bitboard = 0u64;

        match player{
            Color::White => {
                king_bitboard |= self.piece_bitboards[WHITE_KING as usize - 1];
            },
            Color::Black => {
                king_bitboard |= self.piece_bitboards[BLACK_KING as usize - 1];
            }
        }

        let king_square = king_bitboard.trailing_zeros() as u8;

        return self.is_square_attacked(king_square, player.opposite());
    }

    pub fn is_square_attacked(&self, square: u8, attacker_color: Color) -> bool {
        // CASES:
        // 1. Is being attacked by a pawn?
        // 2. Is being attacked by an enemy knight?
        // 3. ¿Le ataca un alfil/dama enemigo? (usar get_bishop_attacks)
        // 4. ¿Le ataca una torre/dama enemiga? (usar get_rook_attacks)
        // 5. Is being attacked by the king?

        match attacker_color{
            Color::White =>{
                // Pawn
                let enemy_pawn_piece = WHITE_PAWN; 
                let enemy_pawns = self.piece_bitboards[enemy_pawn_piece as usize - 1];

                let not_a_file: u64 = 0xFEFEFEFEFEFEFEFE;
                let not_h_file: u64 = 0x7F7F7F7F7F7F7F7F; 
                let bitboard = 1u64 << square;
                let mut pawn_bitboard = 0u64;

                if (bitboard >> 9) & not_h_file == 0 {
                    pawn_bitboard |= bitboard >> 7;
                }
                else if (bitboard >> 7) & not_a_file == 0 {
                    pawn_bitboard |= bitboard >> 9;
                }
                else{
                    pawn_bitboard |= bitboard >> 7;
                    pawn_bitboard |= bitboard >> 9;
                }

                if enemy_pawns & pawn_bitboard != 0 { return true; }

                // Knight
                let enemy_knight_piece = WHITE_KNIGHT; 
                let enemy_knights = self.piece_bitboards[enemy_knight_piece as usize - 1];
                if KNIGHT_ATTACKS[square as usize] & enemy_knights != 0 { return true; }

                // Bishop
                let enemy_bishop_piece = WHITE_BISHOP;
                let enemy_bishops = self.piece_bitboards[enemy_bishop_piece as usize -1];

                let blockers = self.all_pieces & BISHOP_MASKS[square as usize];
                let b_offset = BISHOP_OFFSETS[square as usize];
                let b_shifts = BISHOP_SHIFTS[square as usize];
                let b_magic = BISHOP_MAGICS[square as usize];

                let b_hash = (blockers.wrapping_mul(b_magic) >> b_shifts) as usize;

                let bishop_attacks = BISHOP_ATTACKS_TABLE[b_hash + b_offset];

                if enemy_bishops & bishop_attacks != 0 { return true; }

                // Rook
                let enemy_rook_piece = WHITE_ROOK;
                let enemy_rooks = self.piece_bitboards[enemy_rook_piece as usize -1];

                let blockers = self.all_pieces & ROOK_MASKS[square as usize];
                let r_offset = ROOK_OFFSETS[square as usize];
                let r_shifts = ROOK_SHIFTS[square as usize];
                let r_magic = ROOK_MAGICS[square as usize];

                let r_hash = (blockers.wrapping_mul(r_magic) >> r_shifts) as usize;

                let rook_attacks = ROOK_ATTACKS_TABLE[r_hash + r_offset];

                if enemy_rooks & rook_attacks != 0 { return true; }

                // Queen
                let enemy_queen_piece = WHITE_QUEEN;
                let enemy_queens = self.piece_bitboards[enemy_queen_piece as usize -1];

                if enemy_queens & rook_attacks != 0 || enemy_queens & bishop_attacks != 0 { return true; }

                // King
                let enemy_king_piece = WHITE_KING; 
                let enemy_king = self.piece_bitboards[enemy_king_piece as usize - 1];
                if KING_ATTACKS[square as usize] & enemy_king != 0 { return true; }

                return false;
            },
            Color::Black =>{
                // Pawn
                let enemy_pawn_piece = BLACK_PAWN; 
                let enemy_pawns = self.piece_bitboards[enemy_pawn_piece as usize - 1];

                let not_a_file: u64 = 0xFEFEFEFEFEFEFEFE;
                let not_h_file: u64 = 0x7F7F7F7F7F7F7F7F; 
                let bitboard = 1u64 << square;
                let mut pawn_bitboard = 0u64;

                if (bitboard << 9) & not_a_file == 0 {
                    pawn_bitboard |= bitboard << 7;
                }
                else if (bitboard << 7) & not_h_file == 0 {
                    pawn_bitboard |= bitboard << 9;
                }
                else{
                    pawn_bitboard |= bitboard << 7;
                    pawn_bitboard |= bitboard << 9;
                }

                if enemy_pawns & pawn_bitboard != 0 { return true; }

                // Knight
                let enemy_knight_piece = BLACK_KNIGHT; 
                let enemy_knights = self.piece_bitboards[enemy_knight_piece as usize - 1];
                if KNIGHT_ATTACKS[square as usize] & enemy_knights != 0 { return true; }

                // Bishop
                let enemy_bishop_piece = BLACK_BISHOP;
                let enemy_bishops = self.piece_bitboards[enemy_bishop_piece as usize -1];

                let blockers = self.all_pieces & BISHOP_MASKS[square as usize];
                let b_offset = BISHOP_OFFSETS[square as usize];
                let b_shifts = BISHOP_SHIFTS[square as usize];
                let b_magic = BISHOP_MAGICS[square as usize];

                let b_hash = (blockers.wrapping_mul(b_magic) >> b_shifts) as usize;

                let bishop_attacks = BISHOP_ATTACKS_TABLE[b_hash + b_offset];

                if enemy_bishops & bishop_attacks != 0 { return true; }

                // Rook
                let enemy_rook_piece = BLACK_ROOK;
                let enemy_rooks = self.piece_bitboards[enemy_rook_piece as usize -1];

                let blockers = self.all_pieces & ROOK_MASKS[square as usize];
                let r_offset = ROOK_OFFSETS[square as usize];
                let r_shifts = ROOK_SHIFTS[square as usize];
                let r_magic = ROOK_MAGICS[square as usize];

                let r_hash = (blockers.wrapping_mul(r_magic) >> r_shifts) as usize;

                let rook_attacks = ROOK_ATTACKS_TABLE[r_hash + r_offset];

                if enemy_rooks & rook_attacks != 0 { return true; }

                // Queen
                let enemy_queen_piece = BLACK_QUEEN;
                let enemy_queens = self.piece_bitboards[enemy_queen_piece as usize -1];

                if enemy_queens & rook_attacks != 0 || enemy_queens & bishop_attacks != 0 { return true; }

                // King
                let enemy_king_piece = BLACK_KING; 
                let enemy_king = self.piece_bitboards[enemy_king_piece as usize - 1];
                if KING_ATTACKS[square as usize] & enemy_king != 0 { return true; }

                return false;
            },
        }
    }
}