use crate::board::Color::{Black, White};
use crate::board::movegen::masks::bishop_masks::{BISHOP_ATTACKS_TABLE, BISHOP_MAGICS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_SHIFTS};
use crate::board::{Board, Color};
use crate::board::types::{
    Move, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, EMPTY
};

pub const WHITE_SQUARES_MASK: u64 = 0xAA55_AA55_AA55_AA55;
pub const BLACK_SQUARES_MASK: u64 = 0x55AA_55AA_55AA_55AA;

pub const KNIGHT_PST: [i32; 64] = [
    // Row 1 (a1 - h1)
    -50, -40, -30, -30, -30, -30, -40, -50,
    // Row 2 (a2 - h2)
    -40, -20,   0,   5,   5,   0, -20, -40,
    // Row 3 (a3 - h3)
    -30,   5,  10,  15,  15,  10,   5, -30,
    // Row 4 (a4 - h4)
    -30,   0,  15,  20,  20,  15,   0, -30,
    // Row 5 (a5 - h5)
    -30,   5,  15,  20,  20,  15,   5, -30,
    // Row 6 (a6 - h6)
    -30,   0,  10,  15,  15,  10,   0, -30,
    // Row 7 (a7 - h7)
    -40, -20,   0,   0,   0,   0, -20, -40,
    // Row 8 (a8 - h8)
    -50, -40, -30, -30, -30, -30, -40, -50,
];

// The piece-square table must be done into two parts.
// In the static one we establish normally good squares for the bishops (fianchetos, center active squares, etc)
// In the second one, the dynamic one, we must take into account the actual possition in order to calculate the true value 
// (done in evaluate())
pub const BISHOP_PST: [i32; 64] = [
    // Row 1 (a1 - h1) -> Penalices being inactive on the initial square
    -20, -10, -10, -10, -10, -10, -10, -20,
    // Row 2 (a2 - h2) -> Bonus in b2/g2 for Fianchetto (+5)
    -10,   5,   0,   0,   0,   0,   5, -10,
    // Row 3 (a3 - h3) -> Active developement
    -10,  10,  10,  10,  10,  10,  10, -10,
    // Row 4 (a4 - h4) -> Domination of the diagonals
    -10,   0,  10,  10,  10,  10,   0, -10,
    // Row 5 (a5 - h5)
    -10,   5,   5,  10,  10,   5,   5, -10,
    // Row 6 (a6 - h6)
    -10,   0,   5,  10,  10,   5,   0, -10,
    // Row 7 (a7 - h7)
    -10,   0,   0,   0,   0,   0,   0, -10,
    // Row 8 (a8 - h8)
    -20, -10, -10, -10, -10, -10, -10, -20,
];

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 2000;


impl Board{
    pub fn evaluate(&self) -> i32{
        let (own_color, own_bitboard, enemy_color, enemy_bitboard) = match self.side_to_move {
            Color::White => (White, self.white_pieces, Black, self.black_pieces),
            Color::Black => (Black, self.black_pieces, White, self.white_pieces),
        };

        self.material_value(own_bitboard, own_color) - self.material_value(enemy_bitboard, enemy_color)
    }

    // Sums the material value of every piece on the given bitboard (e.g. self.white_pieces).
    fn material_value(&self, mut bitboard: u64, color: Color) -> i32 {
        // First calculate the bonus for the pair of bishops. If not score is 0
        let mut total = self.pair_of_bishops(color);

        while bitboard != 0 {
            let square = bitboard.trailing_zeros();

            total += self.piece_value(self.squares[square as usize], square as u8);

            bitboard &= bitboard - 1;
        }

        total
    }

    // Scores a capture
    pub fn score_move(&self, mv: &Move) -> i32 {
        if self.is_capture(mv) {
            let attacker = self.squares[mv.origin as usize];
            let victim = self.squares[mv.destination as usize];

            // MVV-LVA puntuation. attacker uses a small ordinal rank here, not piece_value's
            // material scale -- piece_value(KING) is large enough (2000) that it could
            // outweigh victim_value * 10 for a low-value victim (e.g. a king capturing a
            // pawn: 100*10 - 2000 = -1000), scoring a real capture below a quiet move (0).
            // A cheap ordinal rank can never do that regardless of how piece_value is tuned.
            return (self.piece_value(victim, mv.destination) * 10) - self.attacker_rank(attacker);
        }

        0
    }

    pub fn piece_value(&self, piece: u8, square: u8) -> i32{
        match piece{
            WHITE_PAWN | BLACK_PAWN => PAWN_VALUE,
            WHITE_KNIGHT | BLACK_KNIGHT => KNIGHT_VALUE,
            WHITE_BISHOP | BLACK_BISHOP => BISHOP_VALUE + self.bishop_value(piece, square),
            WHITE_ROOK | BLACK_ROOK => ROOK_VALUE,
            WHITE_QUEEN | BLACK_QUEEN => QUEEN_VALUE,
            WHITE_KING | BLACK_KING => KING_VALUE,
            _ => 0,
        }
    }

    // If the white bishop pair is conserved we give a bonus
    fn pair_of_bishops(&self, color: Color) -> i32 {
        let mut bonus = 0;

        if color == White{
            if self.piece_bitboards[(WHITE_BISHOP - 1) as usize].count_ones() == 2 {
                bonus += 40;
            }
        }
        else{
            if self.piece_bitboards[(BLACK_BISHOP - 1) as usize].count_ones() == 2 {
                bonus += 40;
            }
        }
        
        bonus
    }

    fn bishop_value(&self, piece: u8, square: u8) -> i32 {
        let blockers = self.all_pieces & BISHOP_MASKS[square as usize];
        let magic = BISHOP_MAGICS[square as usize];
        let shift = BISHOP_SHIFTS[square as usize];
        let offset = BISHOP_OFFSETS[square as usize];
        
        let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

        let mut valid_attacks = BISHOP_ATTACKS_TABLE[hash + offset];

        let mut bonus = BISHOP_PST[square as usize];

        // We must check which color is the actual square
        let own_square_color_mask = if (1u64 << square) & WHITE_SQUARES_MASK != 0 {
            WHITE_SQUARES_MASK
        } else {
            BLACK_SQUARES_MASK
        };

        if piece == WHITE_BISHOP {
            valid_attacks &= !self.white_pieces;

            // If we have too many pawns in that bishop color the value of the bishop decrease
            let white_pawn_in_white_bitboard = self.piece_bitboards[(WHITE_PAWN - 1) as usize] & own_square_color_mask;
            let white_pawn_in_white = white_pawn_in_white_bitboard.count_ones() as i32;
            
            bonus -= white_pawn_in_white*5;
        }
        // Black bishop
        else {
            valid_attacks &= !self.black_pieces;

            // If we have too many pawns in that bishop color the value of the bishop decrease
            let black_pawn_in_black_bitboard = self.piece_bitboards[(BLACK_PAWN - 1) as usize] & own_square_color_mask;
            let black_pawn_in_black = black_pawn_in_black_bitboard.count_ones() as i32;
            
            bonus -= black_pawn_in_black*5;
        }

        // We give a bonus for more squares covered by the bishop
        bonus += (valid_attacks.count_ones() as i32 - 7) * 5;

        bonus
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