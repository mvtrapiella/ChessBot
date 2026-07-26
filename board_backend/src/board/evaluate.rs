use crate::board::{Board, Color};
use super::types::{
    Move, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, NO_SQUARE,
};

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;

impl Board{
    pub fn evaluate(&self) -> i32{
        let (own_bitboard, enemy_bitboard) = match self.side_to_move {
            Color::White => (self.white_pieces, self.black_pieces),
            Color::Black => (self.black_pieces, self.white_pieces),
        };

        self.material_value(own_bitboard) - self.material_value(enemy_bitboard)
    }

    // Sums the material value of every piece on the given bitboard (e.g. self.white_pieces).
    fn material_value(&self, mut bitboard: u64) -> i32 {
        let mut total = 0;

        while bitboard != 0 {
            let square = bitboard.trailing_zeros();

            total += match self.squares[square as usize] {
                WHITE_PAWN | BLACK_PAWN => PAWN_VALUE,
                WHITE_KNIGHT | BLACK_KNIGHT => KNIGHT_VALUE,
                WHITE_BISHOP | BLACK_BISHOP => BISHOP_VALUE,
                WHITE_ROOK | BLACK_ROOK => ROOK_VALUE,
                WHITE_QUEEN | BLACK_QUEEN => QUEEN_VALUE,
                _ => 0,
            };

            bitboard &= bitboard - 1;
        }

        total
    }
}