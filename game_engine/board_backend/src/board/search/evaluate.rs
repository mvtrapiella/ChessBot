use crate::board::{Board, Color};
use crate::board::types::{
    Move, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, EMPTY
};

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 2000;


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

            total += self.piece_value(self.squares[square as usize]);

            bitboard &= bitboard - 1;
        }

        total
    }

    pub fn score_move(&self, mv: &Move) -> i32 {
        if self.is_capture(mv) {
            let attacker = self.squares[mv.origin as usize];
            let victim = self.squares[mv.destination as usize];

            // MVV-LVA puntuation. attacker uses a small ordinal rank here, not piece_value's
            // material scale -- piece_value(KING) is large enough (2000) that it could
            // outweigh victim_value * 10 for a low-value victim (e.g. a king capturing a
            // pawn: 100*10 - 2000 = -1000), scoring a real capture below a quiet move (0).
            // A cheap ordinal rank can never do that regardless of how piece_value is tuned.
            return (self.piece_value(victim) * 10) - self.attacker_rank(attacker);
        }

        0
    }

    pub fn piece_value(&self, piece: u8) -> i32{
        match piece{
            WHITE_PAWN | BLACK_PAWN => PAWN_VALUE,
            WHITE_KNIGHT | BLACK_KNIGHT => KNIGHT_VALUE,
            WHITE_BISHOP | BLACK_BISHOP => BISHOP_VALUE,
            WHITE_ROOK | BLACK_ROOK => ROOK_VALUE,
            WHITE_QUEEN | BLACK_QUEEN => QUEEN_VALUE,
            WHITE_KING | BLACK_KING => KING_VALUE,
            _ => 0,
        }
    }

    // Ordinal "cheapness" rank used only for the attacker side of MVV-LVA move ordering --
    // deliberately not piece_value's material scale, so tuning piece values can never
    // distort move ordering (see score_move).
    fn attacker_rank(&self, piece: u8) -> i32 {
        match piece {
            WHITE_PAWN | BLACK_PAWN => 1,
            WHITE_KNIGHT | BLACK_KNIGHT => 2,
            WHITE_BISHOP | BLACK_BISHOP => 2,
            WHITE_ROOK | BLACK_ROOK => 3,
            WHITE_QUEEN | BLACK_QUEEN => 4,
            WHITE_KING | BLACK_KING => 5,
            _ => 0,
        }
    }

    pub fn is_capture(&self, m: &Move) -> bool{
        let moved_piece = self.squares[m.origin as usize];
        let is_pawn = moved_piece == WHITE_PAWN || moved_piece == BLACK_PAWN;

        let is_en_passant = is_pawn
            && self.squares[m.destination as usize] == EMPTY
            && m.destination == self.en_passant_square;
        return self.squares[m.destination as usize] != 0 || is_en_passant;
    }
}