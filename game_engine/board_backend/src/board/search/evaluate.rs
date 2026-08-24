use crate::board::Color::{Black, White};
use crate::board::make_move::{WHITE_SHORT_CASTLE, WHITE_LONG_CASTLE, BLACK_SHORT_CASTLE, BLACK_LONG_CASTLE};
use crate::board::movegen::masks::bishop_masks::{BISHOP_ATTACKS_TABLE, BISHOP_MAGICS, BISHOP_MASKS, BISHOP_OFFSETS, BISHOP_SHIFTS};
use crate::board::movegen::masks::rook_masks::{ROOK_ATTACKS_TABLE, ROOK_MAGICS, ROOK_MASKS, ROOK_OFFSETS, ROOK_SHIFTS};
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

pub const ROOK_PST: [i32; 64] = [
    // Rank 1 (a1 - h1) -> Starting position: prefers centralizing on d1/e1 (+5)
     0,  0,  0,  5,  5,  0,  0,  0,
    // Rank 2 (a2 - h2) -> Penalized edges (-5)
    -5,  0,  0,  0,  0,  0,  0, -5,
    // Rank 3 (a3 - h3)
    -5,  0,  0,  0,  0,  0,  0, -5,
    // Rank 4 (a4 - h4)
    -5,  0,  0,  0,  0,  0,  0, -5,
    // Rank 5 (a5 - h5)
    -5,  0,  0,  0,  0,  0,  0, -5,
    // Rank 6 (a6 - h6)
    -5,  0,  0,  0,  0,  0,  0, -5,
    // Rank 7 (a7 - h7) -> Major bonus for reaching the 7th rank (+10 on d7/e7)
     5, 10, 10, 10, 10, 10, 10,  5,
    // Rank 8 (a8 - h8) -> Control of the 8th rank
     0,  0,  0,  0,  0,  0,  0,  0,
];

// In order to nor have a table one for white and other for black pawns we do the XOR operation of the square by 56
// This is becuse a cell is represented by an 8 bits number where: 00 [unused] 000 [file] 000 [column]
// The last three bits go from 0 (000) - 7 (111) so this represents the column. Why 56?
// This is because 56 is 111 000 in binary so it only changes the file bits. By doing XOR we invert the bits so if it was
// a black pawn on 55 cell (110 111), the pawn on h7 after doing XOR with 111 000 we obtain 001 111 that is the number 15 that is 
// the corresponding pawn on h2
pub const PAWN_PST: [i32; 64] = [
    // Row 1 (a1 - h1) -> Imposible have pawns on the first row
     0,  0,  0,  0,  0,  0,  0,  0,
    // Row 2 (a2 - h2) -> Intial position, penalics d/e pawns for not advancing
     0,  0,  0,-20,-20,  0,  0,  0,
    // Row 3 (a3 - h3) -> Small advance
     5, -5,-10,  0,  0,-10, -5,  5,
    // Row 4 (a4 - h4) -> Center domination (+20 in d4/e4)
     0,  0,  0, 20, 20,  0,  0,  0,
    // Row 5 (a5 - h5) -> Invade the enemy field (+25 in d5/e5)
     5,  5, 10, 25, 25, 10,  5,  5,
    // Row 6 (a6 - h6) -> Passed pawn
    10, 10, 20, 30, 30, 20, 10, 10,
    // Row 7 (a7 - h7) -> Passed pan + (+50)
    50, 50, 50, 50, 50, 50, 50, 50,
    // Row 8 (a8 - h8) -> Imposible (here the pawn has already converted into another piece)
     0,  0,  0,  0,  0,  0,  0,  0,
];

pub const QUEEN_PST: [i32; 64] = [
    // Rank 1 (a1 - h1) -> Starting position (d1) and back rank
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    // Rank 2 (a2 - h2) -> Safe development squares
    -10,   0,   0,   0,   0,   0,   0, -10,
    // Rank 3 (a3 - h3)
    -10,   0,   5,   5,   5,   5,   0, -10,
    // Rank 4 (a4 - h4) -> Center control
     -5,   0,   5,   5,   5,   5,   0,  -5,
    // Rank 5 (a5 - h5)
      0,   0,   5,   5,   5,   5,   0,  -5,
    // Rank 6 (a6 - h6)
    -10,   5,   5,   5,   5,   5,   0, -10,
    // Rank 7 (a7 - h7) -> Infiltration
    -10,   0,   5,   0,   0,   0,   0, -10,
    // Rank 8 (a8 - h8) -> Enemy back rank
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

pub const KING_MG_PST: [i32; 64] = [
    // Rank 1 (a1 - h1) -> Rewards castled positions (g1, b1, c1)
     20,  30,  10,   0,   0,  10,  30,  20,
    // Rank 2 (a2 - h2)
     20,  20,   0,   0,   0,   0,  20,  20,
    // Rank 3 (a3 - h3) -> Penalizes stepping out too early
    -10, -20, -20, -20, -20, -20, -20, -10,
    // Rank 4 (a4 - h4)
    -20, -30, -30, -40, -40, -30, -30, -20,
    // Rank 5 (a5 - h5)
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 6 (a6 - h6)
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 7 (a7 - h7)
    -30, -40, -40, -50, -50, -40, -40, -30,
    // Rank 8 (a8 - h8)
    -30, -40, -40, -50, -50, -40, -40, -30,
];

pub const KING_EG_PST: [i32; 64] = [
    // Rank 1 (a1 - h1) -> Penalizes staying in corners
    -50, -40, -30, -20, -20, -30, -40, -50,
    // Rank 2 (a2 - h2)
    -30, -20, -10,   0,   0, -10, -20, -30,
    // Rank 3 (a3 - h3)
    -30, -10,  20,  30,  30,  20, -10, -30,
    // Rank 4 (a4 - h4) -> High bonus for central control
    -30, -10,  30,  40,  40,  30, -10, -30,
    // Rank 5 (a5 - h5) -> High bonus for central control
    -30, -10,  30,  40,  40,  30, -10, -30,
    // Rank 6 (a6 - h6)
    -30, -10,  20,  30,  30,  20, -10, -30,
    // Rank 7 (a7 - h7)
    -30, -20, -10,   0,   0, -10, -20, -30,
    // Rank 8 (a8 - h8)
    -50, -40, -30, -20, -20, -30, -40, -50,
];

// This is a table with a precomputed masks for the different pawn possitions so we can easily check
// if the pawn is a passed or not pawn
pub const PASSED_PAWN_MASKS: [[u64; 64]; 2] = [
  [0x0303030303030300, 0x0707070707070700, 0x0e0e0e0e0e0e0e00, 0x1c1c1c1c1c1c1c00, 
   0x3838383838383800, 0x7070707070707000, 0xe0e0e0e0e0e0e000, 0xc0c0c0c0c0c0c000, 
   0x0303030303030000, 0x0707070707070000, 0x0e0e0e0e0e0e0000, 0x1c1c1c1c1c1c0000, 
   0x3838383838380000, 0x7070707070700000, 0xe0e0e0e0e0e00000, 0xc0c0c0c0c0c00000, 
   0x0303030303000000, 0x0707070707000000, 0x0e0e0e0e0e000000, 0x1c1c1c1c1c000000, 
   0x3838383838000000, 0x7070707070000000, 0xe0e0e0e0e0000000, 0xc0c0c0c0c0000000, 
   0x0303030300000000, 0x0707070700000000, 0x0e0e0e0e00000000, 0x1c1c1c1c00000000, 
   0x3838383800000000, 0x7070707000000000, 0xe0e0e0e000000000, 0xc0c0c0c000000000, 
   0x0303030000000000, 0x0707070000000000, 0x0e0e0e0000000000, 0x1c1c1c0000000000, 
   0x3838380000000000, 0x7070700000000000, 0xe0e0e00000000000, 0xc0c0c00000000000, 
   0x0303000000000000, 0x0707000000000000, 0x0e0e000000000000, 0x1c1c000000000000, 
   0x3838000000000000, 0x7070000000000000, 0xe0e0000000000000, 0xc0c0000000000000, 
   0x0300000000000000, 0x0700000000000000, 0x0e00000000000000, 0x1c00000000000000, 
   0x3800000000000000, 0x7000000000000000, 0xe000000000000000, 0xc000000000000000, 
   0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 
   0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 
   ],
  [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 
   0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 
   0x0000000000000003, 0x0000000000000007, 0x000000000000000e, 0x000000000000001c, 
   0x0000000000000038, 0x0000000000000070, 0x00000000000000e0, 0x00000000000000c0, 
   0x0000000000000303, 0x0000000000000707, 0x0000000000000e0e, 0x0000000000001c1c, 
   0x0000000000003838, 0x0000000000007070, 0x000000000000e0e0, 0x000000000000c0c0, 
   0x0000000000030303, 0x0000000000070707, 0x00000000000e0e0e, 0x00000000001c1c1c, 
   0x0000000000383838, 0x0000000000707070, 0x0000000000e0e0e0, 0x0000000000c0c0c0, 
   0x0000000003030303, 0x0000000007070707, 0x000000000e0e0e0e, 0x000000001c1c1c1c, 
   0x0000000038383838, 0x0000000070707070, 0x00000000e0e0e0e0, 0x00000000c0c0c0c0, 
   0x0000000303030303, 0x0000000707070707, 0x0000000e0e0e0e0e, 0x0000001c1c1c1c1c, 
   0x0000003838383838, 0x0000007070707070, 0x000000e0e0e0e0e0, 0x000000c0c0c0c0c0, 
   0x0000030303030303, 0x0000070707070707, 0x00000e0e0e0e0e0e, 0x00001c1c1c1c1c1c, 
   0x0000383838383838, 0x0000707070707070, 0x0000e0e0e0e0e0e0, 0x0000c0c0c0c0c0c0, 
   0x0003030303030303, 0x0007070707070707, 0x000e0e0e0e0e0e0e, 0x001c1c1c1c1c1c1c, 
   0x0038383838383838, 0x0070707070707070, 0x00e0e0e0e0e0e0e0, 0x00c0c0c0c0c0c0c0, 
   ],
];

pub const ISOLATED_MASKS: [u64; 8] = [
    0x0202_0202_0202_0202, // Column A -> only checks Column B
    0x0505_0505_0505_0505, // Column B -> checks Columns A and C
    0x0A0A_0A0A_0A0A_0A0A, // Column C -> checks Columns B and D
    0x1414_1414_1414_1414, // Column D -> checks Columns C and E
    0x2828_2828_2828_2828, // Column E -> checks Columns D and F
    0x5050_5050_5050_5050, // Column F -> checks Columns E and G
    0xA0A0_A0A0_A0A0_A0A0, // Column G -> checks Columns F and H
    0x4040_4040_4040_4040, // Column H -> only checks Column G
];

pub const FILE_MASKS: [u64; 8] = [
    0x0101_0101_0101_0101, // Column A
    0x0202_0202_0202_0202, // Column B
    0x0404_0404_0404_0404, // Column C
    0x0808_0808_0808_0808, // Column D
    0x1010_1010_1010_1010, // Column E
    0x2020_2020_2020_2020, // Column F
    0x4040_4040_4040_4040, // Column G
    0x8080_8080_8080_8080, // Column H
];

pub const RANK_MASKS: [u64; 8] = [
    0x0000_0000_0000_00FF, // Rank 1 (a1 - h1)
    0x0000_0000_0000_FF00, // Rank 2 (a2 - h2)
    0x0000_0000_00FF_0000, // Rank 3 (a3 - h3)
    0x0000_0000_FF00_0000, // Rank 4 (a4 - h4)
    0x0000_00FF_0000_0000, // Rank 5 (a5 - h5)
    0x0000_FF00_0000_0000, // Rank 6 (a6 - h6)
    0x00FF_0000_0000_0000, // Rank 7 (a7 - h7)
    0xFF00_0000_0000_0000, // Rank 8 (a8 - h8)
];

const PASSED_PAWN_MG_BY_RANK: [i32; 8] = [0, 5, 10, 15, 25, 40, 60, 0];
const PASSED_PAWN_EG_BY_RANK: [i32; 8] = [0, 10, 20, 35, 60, 100, 180, 0];

const WHITE_MINOR_HOME_MASK: u64 = (1u64 << 1) | (1u64 << 2) | (1u64 << 5) | (1u64 << 6);   // b1, c1, f1, g1
const BLACK_MINOR_HOME_MASK: u64 = (1u64 << 57) | (1u64 << 58) | (1u64 << 61) | (1u64 << 62); // b8, c8, f8, g8

pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 300;
pub const BISHOP_VALUE: i32 = 300;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;
pub const KING_VALUE: i32 = 2000;

pub const MAX_PHASE: i32 = 24;

// Weight of phase
pub const KNIGHT_PHASE: i32 = 1;
pub const BISHOP_PHASE: i32 = 1;
pub const ROOK_PHASE:   i32 = 2;
pub const QUEEN_PHASE:  i32 = 4;


impl Board{
    pub fn evaluate(&self, moves_counter: u32) -> i32{
        let phase = self.calculate_game_phase();
        let (own_color, own_bitboard, enemy_color, enemy_bitboard) = match self.side_to_move {
            Color::White => (White, self.white_pieces, Black, self.black_pieces),
            Color::Black => (Black, self.black_pieces, White, self.white_pieces),
        };

        self.material_value(own_bitboard, own_color, phase, moves_counter) - self.material_value(enemy_bitboard, enemy_color, phase, moves_counter)
    }

    fn calculate_game_phase(&self) -> i32 {
        let mut phase: i32 = 0;

        // Count Knights
        phase += (self.piece_bitboards[(WHITE_KNIGHT - 1) as usize].count_ones() as i32 + self.piece_bitboards[(BLACK_KNIGHT - 1) as usize].count_ones() as i32)*KNIGHT_PHASE;
        
        // Count Bishops
        phase += (self.piece_bitboards[(WHITE_BISHOP - 1) as usize].count_ones() as i32 + self.piece_bitboards[(BLACK_BISHOP - 1) as usize].count_ones() as i32)*BISHOP_PHASE;
    
        // Count Rooks
        phase += (self.piece_bitboards[(WHITE_ROOK - 1) as usize].count_ones() as i32 + self.piece_bitboards[(BLACK_ROOK - 1) as usize].count_ones() as i32)*ROOK_PHASE;
    
        // Count Queens
        phase += (self.piece_bitboards[(WHITE_QUEEN - 1) as usize].count_ones() as i32 + self.piece_bitboards[(BLACK_QUEEN - 1) as usize].count_ones() as i32)*QUEEN_PHASE;

        // We use the min in case there are promotions so the max_phase is never exceded
        phase.min(MAX_PHASE)
    }

    fn interpolate_score(&self, mg_score: i32, eg_score: i32, phase: i32) -> i32 {
        let mg_weight = phase;
        let eg_weight = MAX_PHASE - phase;

        (mg_score * mg_weight + eg_score * eg_weight) / MAX_PHASE
    }

    // Sums the material value of every piece on the given bitboard (e.g. self.white_pieces).
    fn material_value(&self, mut bitboard: u64, color: Color, phase: i32, moves_counter: u32) -> i32 {
        // First calculate the bonus for the pair of bishops. If not score is 0
        let mut total = self.pair_of_bishops(color);

        // Then check if the towers are connected
        total += self.rooks_connected(color);

        while bitboard != 0 {
            let square = bitboard.trailing_zeros();

            total += self.piece_value(self.squares[square as usize], square as u8, phase, moves_counter);

            bitboard &= bitboard - 1;
        }

        total
    }

    // Scores a capture
    pub fn score_move(&self, mv: &Move, moves_counter: u32) -> i32 {
        if self.is_capture(mv) {
            let attacker = self.squares[mv.origin as usize];
            let victim = self.squares[mv.destination as usize];
            let phase = self.calculate_game_phase();

            // MVV-LVA puntuation. attacker uses a small ordinal rank here, not piece_value's
            // material scale -- piece_value(KING) is large enough (2000) that it could
            // outweigh victim_value * 10 for a low-value victim (e.g. a king capturing a
            // pawn: 100*10 - 2000 = -1000), scoring a real capture below a quiet move (0).
            // A cheap ordinal rank can never do that regardless of how piece_value is tuned.
            return (self.piece_value(victim, mv.destination, phase, moves_counter) * 10) - self.attacker_rank(attacker);
        }

        0
    }

    pub fn piece_value(&self, piece: u8, square: u8, phase: i32, moves_counter: u32) -> i32{
        match piece{
            WHITE_PAWN | BLACK_PAWN => PAWN_VALUE + self.pawn_value(piece, square, phase),
            WHITE_KNIGHT | BLACK_KNIGHT => KNIGHT_VALUE + self.knight_value(square),
            WHITE_BISHOP | BLACK_BISHOP => BISHOP_VALUE + self.bishop_value(piece, square),
            WHITE_ROOK | BLACK_ROOK => ROOK_VALUE
                + self.trapped_rook_penalty(piece, square)
                + self.undeveloped_rook_penalty(piece, square, moves_counter)
                + self.rook_value(piece, square),
            WHITE_QUEEN | BLACK_QUEEN => QUEEN_VALUE + self.early_queen_penalty(piece, square, moves_counter) + self.queen_value(piece, square),
            WHITE_KING | BLACK_KING => KING_VALUE
                + self.king_value(piece, square, phase)
                + self.pawn_shield_penalty(piece, square, phase)
                + self.lost_castling_penalty(piece, square, moves_counter),
            _ => 0,
        }
    }

    fn lost_castling_penalty(&self, piece: u8, square: u8, moves_counter: u32) -> i32 {
        const CASTLE_GRACE_PERIOD_PLIES: u32 = 30;

        if moves_counter >= CASTLE_GRACE_PERIOD_PLIES {
            return 0;
        }

        let is_white = piece == WHITE_KING;
        let (castle_rights_mask, kingside_square, queenside_square) = if is_white {
            (WHITE_SHORT_CASTLE | WHITE_LONG_CASTLE, 6, 2)
        } else {
            (BLACK_SHORT_CASTLE | BLACK_LONG_CASTLE, 62, 58)
        };

        if square == kingside_square || square == queenside_square {
            return 0;
        }

        if self.castling_rights & castle_rights_mask == 0 {
            -30
        } else {
            0
        }
    }

    fn king_value(&self, piece: u8, square: u8, phase: i32) -> i32 {
        let mut bonus = 0;

        let index = if piece == WHITE_KING {
            square
        }
        else {
            square^56
        };

        bonus += self.interpolate_score(KING_MG_PST[index as usize], KING_EG_PST[index as usize], phase);

        bonus
    }

    fn pawn_shield_penalty(&self, piece: u8, square: u8, phase: i32) -> i32 {
        // Only matters while there's still enough material for a mating attack to be a real threat
        if phase <= 10 {
            return 0;
        }

        let is_white = piece == WHITE_KING;
        let own_pawns = if is_white {
            self.piece_bitboards[(WHITE_PAWN - 1) as usize]
        } else {
            self.piece_bitboards[(BLACK_PAWN - 1) as usize]
        };

        // Only check when the king is actually on a castled square -- an uncastled
        // king stuck in the center has much bigger problems than a missing pawn.
        let shield_mask: u64 = if is_white {
            match square {
                6 => (1u64 << 13) | (1u64 << 14) | (1u64 << 15), // Kg1 -> f2, g2, h2
                2 => (1u64 << 8)  | (1u64 << 9)  | (1u64 << 10), // Kc1 -> a2, b2, c2
                _ => return 0,
            }
        } else {
            match square {
                62 => (1u64 << 53) | (1u64 << 54) | (1u64 << 55), // Kg8 -> f7, g7, h7
                58 => (1u64 << 48) | (1u64 << 49) | (1u64 << 50), // Kc8 -> a7, b7, c7
                _ => return 0,
            }
        };

        let missing = 3 - (own_pawns & shield_mask).count_ones() as i32;
        -(missing * 15)
    }

    fn early_queen_penalty(&self, piece: u8, square: u8, moves_counter: u32) -> i32 {
        const QUEEN_DEVELOPMENT_GRACE_PERIOD_PLIES: u32 = 20;

        if moves_counter >= QUEEN_DEVELOPMENT_GRACE_PERIOD_PLIES {
            return 0;
        }

        let (home_square, minor_home_mask, own_knights, own_bishops) = if piece == WHITE_QUEEN {
            (3, WHITE_MINOR_HOME_MASK, self.piece_bitboards[(WHITE_KNIGHT - 1) as usize], self.piece_bitboards[(WHITE_BISHOP - 1) as usize])
        } else {
            (59, BLACK_MINOR_HOME_MASK, self.piece_bitboards[(BLACK_KNIGHT - 1) as usize], self.piece_bitboards[(BLACK_BISHOP - 1) as usize])
        };

        if square == home_square {
            return 0; // still home, nothing to penalize
        }

        // Only counts minors still literally on their own starting square, not just alive
        let undeveloped = ((own_knights | own_bishops) & minor_home_mask).count_ones() as i32;

        -(undeveloped * 10)
    }

    fn queen_value(&self, piece: u8, square: u8) -> i32 {
        let index = if piece == WHITE_QUEEN { square } else { square ^ 56 };
        let mut bonus = QUEEN_PST[index as usize];

        // Rook move
        let rook_blockers = self.all_pieces & ROOK_MASKS[square as usize];
        let rook_magic = ROOK_MAGICS[square as usize];
        let rook_shift = ROOK_SHIFTS[square as usize];
        let rook_offset = ROOK_OFFSETS[square as usize];

        // Bishop move
        let bishop_blockers = self.all_pieces & BISHOP_MASKS[square as usize];
        let bishop_magic = BISHOP_MAGICS[square as usize];
        let bishop_shift = BISHOP_SHIFTS[square as usize];
        let bishop_offset = BISHOP_OFFSETS[square as usize];

        let rook_hash = (rook_blockers.wrapping_mul(rook_magic) >> rook_shift) as usize;

        let mut valid_attacks = ROOK_ATTACKS_TABLE[rook_hash + rook_offset];

        let bishop_hash = (bishop_blockers.wrapping_mul(bishop_magic) >> bishop_shift) as usize;

        valid_attacks |= BISHOP_ATTACKS_TABLE[bishop_hash + bishop_offset];

        let rank= square / 8;
        let enemy_king_square = if piece == WHITE_QUEEN {
            self.piece_bitboards[(BLACK_KING - 1) as usize].trailing_zeros() as u8
        } else {
            self.piece_bitboards[(WHITE_KING - 1) as usize].trailing_zeros() as u8
        };
        let enemy_king_column = enemy_king_square % 8;
        let enemy_king_rank = enemy_king_square / 8;

        // If the queen is pointing to the king direction
        let king_attack_mask = FILE_MASKS[enemy_king_column as usize] | RANK_MASKS[enemy_king_rank as usize];

        // If the queen is pointing towards the king
        if king_attack_mask & (1u64 << square) != 0 {
            bonus += 10;
        } 

        if piece == WHITE_QUEEN {
            valid_attacks &= !self.white_pieces;

            // Bonus if the queen is in the 7th rank if either the king is in 8th rank or there are pawns on the 7th rank
            if rank == 6 && 
            (self.piece_bitboards[(BLACK_KING - 1) as usize] & RANK_MASKS[7] != 0 
            || self.piece_bitboards[(BLACK_PAWN - 1) as usize] & RANK_MASKS[rank as usize] != 0){
                bonus += 20;
            }
        }
        else{
            valid_attacks &= !self.black_pieces;

            // Bonus if the queen is in the 2th rank if either the king is in 1th rank or there are pawns on the 2th rank
            if rank == 1 && 
            (self.piece_bitboards[(WHITE_KING - 1) as usize] & RANK_MASKS[0] != 0 
            || self.piece_bitboards[(WHITE_PAWN - 1) as usize] & RANK_MASKS[rank as usize] != 0){
                bonus += 20;
            }
        }

        // Bonus for movility
        bonus += (valid_attacks.count_ones() as i32 - 14)*1;

        bonus
    }

    // If the white bishop pair is conserved we give a bonus
    fn rooks_connected(&self, color: Color) -> i32 {
        let rooks = if color == White {
            self.piece_bitboards[(WHITE_ROOK - 1) as usize]
        } else {
            self.piece_bitboards[(BLACK_ROOK - 1) as usize]
        };

        if rooks.count_ones() != 2 {
            return 0;
        }

        let first_square = rooks.trailing_zeros() as u8;
        let second_square = (rooks & (rooks - 1)).trailing_zeros() as u8;

        if first_square / 8 == second_square / 8 {
            20
        } else {
            0
        }
    }

    fn trapped_rook_penalty(&self, piece: u8, square: u8) -> i32 {
        let is_white = piece == WHITE_ROOK;

        let king_bitboard = if is_white {
            self.piece_bitboards[(WHITE_KING - 1) as usize]
        } else {
            self.piece_bitboards[(BLACK_KING - 1) as usize]
        };
        let king_square = king_bitboard.trailing_zeros() as u8;

        let own_pawns = if is_white {
            self.piece_bitboards[(WHITE_PAWN - 1) as usize]
        } else {
            self.piece_bitboards[(BLACK_PAWN - 1) as usize]
        };

        let (a1, b1, c1, f1, g1, h1, a2, b2, g2, h2) = if is_white {
            (0, 1, 2, 5, 6, 7, 8, 9, 14, 15)
        } else {
            (56, 57, 58, 61, 62, 63, 48, 49, 54, 55)
        };

        // Trapped on h1/h8: king blocks the rank, h-pawn blocks the file.
        if square == h1 && (king_square == g1 || king_square == f1) {
            let h_pawn_home = own_pawns & (1u64 << h2) != 0;
            let g_pawn_home = own_pawns & (1u64 << g2) != 0;
            if h_pawn_home || g_pawn_home {
                return -50;
            }
        }

        // Trapped on a1/a8: king blocks the rank, a-pawn blocks the file.
        if square == a1 && (king_square == b1 || king_square == c1) {
            let a_pawn_home = own_pawns & (1u64 << a2) != 0;
            let b_pawn_home = own_pawns & (1u64 << b2) != 0;
            if a_pawn_home || b_pawn_home {
                return -50;
            }
        }

        0
    }

    fn undeveloped_rook_penalty(&self, piece: u8, square: u8, moves_counter: u32) -> i32 {
        const ROOK_DEVELOPMENT_GRACE_PERIOD_PLIES: u32 = 24;

        if moves_counter < ROOK_DEVELOPMENT_GRACE_PERIOD_PLIES {
            return 0;
        }

        let is_home_square = if piece == WHITE_ROOK {
            square == 0 || square == 7
        } else {
            square == 56 || square == 63
        };

        if is_home_square { -25 } else { 0 }
    }

    fn rook_value(&self, piece: u8, square: u8) -> i32 {
        let blockers = self.all_pieces & ROOK_MASKS[square as usize];
        let magic = ROOK_MAGICS[square as usize];
        let shift = ROOK_SHIFTS[square as usize];
        let offset = ROOK_OFFSETS[square as usize];
        
        let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

        let mut valid_attacks = ROOK_ATTACKS_TABLE[hash + offset];

        let index = if piece ==WHITE_ROOK {
            square
            
        } else {
            square^56
        };

        let mut bonus = ROOK_PST[index as usize];

        let column = square % 8;
        let rank = square / 8;

        if piece == WHITE_ROOK {
            // Semi-open file
            if FILE_MASKS[column as usize] & self.piece_bitboards[(WHITE_PAWN - 1) as usize] == 0 {
                // Open file
                if FILE_MASKS[column as usize] & self.piece_bitboards[(BLACK_PAWN - 1) as usize] == 0{
                    bonus += 20;
                }
                else {
                    bonus += 15;
                }
            } 

            // Bonus if the rook is in the 7th rank if either the king is in 8th rank or there are pawns on the 7th rank
            if rank == 6 && 
            (self.piece_bitboards[(BLACK_KING - 1) as usize] & RANK_MASKS[7] != 0 
            || self.piece_bitboards[(BLACK_PAWN - 1) as usize] & RANK_MASKS[rank as usize] != 0){
                bonus += 30;
            }

            valid_attacks &= !self.white_pieces;
        }
        else{
            // Semi-open file
            if FILE_MASKS[column as usize] & self.piece_bitboards[(BLACK_PAWN - 1) as usize] == 0 {
                // Open file
                if FILE_MASKS[column as usize] & self.piece_bitboards[(WHITE_PAWN - 1) as usize] == 0{
                    bonus += 20;
                }
                else {
                    bonus += 15;
                }
            }

            // Bonus if the rook is in the 2th rank if either the king is in 1th rank or there are pawns on the 2th rank
                if rank == 1 && 
                (self.piece_bitboards[(WHITE_KING - 1) as usize] & RANK_MASKS[0] != 0 
                || self.piece_bitboards[(WHITE_PAWN - 1) as usize] & RANK_MASKS[rank as usize] != 0){
                    bonus += 30;
                }

            valid_attacks &= !self.black_pieces;
        }

        bonus += (valid_attacks.count_ones() as i32 - 7)*3;

        bonus
    }

    fn pawn_value(&self, piece: u8, square: u8, phase: i32) -> i32 {
        let index = if piece == WHITE_PAWN {
            square
        }
        else{
            square^56
        };

        // Add the value of the PST
        let mut bonus = PAWN_PST[index as usize];

        let column = square % 8;
        let rank = index / 8;

        // Passed pawn: we must check if there is eny enemy pawn on the adyacent or same column
        if piece == WHITE_PAWN {
            if self.piece_bitboards[(BLACK_PAWN - 1) as usize] & PASSED_PAWN_MASKS[White as usize][square as usize] == 0 {
                bonus += self.interpolate_score(PASSED_PAWN_MG_BY_RANK[rank as usize], PASSED_PAWN_EG_BY_RANK[rank as usize], phase);
            }

            // Isolated pawns apply a penalty
            if self.piece_bitboards[(WHITE_PAWN - 1) as usize] & ISOLATED_MASKS[column as usize] == 0 {
                bonus -= 15;
            }

            // Dobled pawns apply a penalty
            if self.piece_bitboards[(WHITE_PAWN - 1) as usize] & FILE_MASKS[column as usize] & !(1u64 << square) != 0 {
                bonus -= 15;
            }
        }
        else {
            if self.piece_bitboards[(WHITE_PAWN - 1) as usize] & PASSED_PAWN_MASKS[Black as usize][square as usize] == 0 {
                bonus += self.interpolate_score(PASSED_PAWN_MG_BY_RANK[rank as usize], PASSED_PAWN_EG_BY_RANK[rank as usize], phase);
            } 

            // Isolated pawns apply a penalty
            if self.piece_bitboards[(BLACK_PAWN - 1) as usize] & ISOLATED_MASKS[column as usize] == 0 {
                bonus -= 15;
            }

            // Dobled pawns apply a penalty
            if self.piece_bitboards[(BLACK_PAWN - 1) as usize] & FILE_MASKS[column as usize] & !(1u64 << square) != 0 {
                bonus -= 15;
            }
        }

        bonus
    }

    fn knight_value(&self, square: u8) -> i32 {
        KNIGHT_PST[square as usize]
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
        let is_pawn: bool = moved_piece == WHITE_PAWN || moved_piece == BLACK_PAWN;

        let is_en_passant = is_pawn
            && self.squares[m.destination as usize] == EMPTY
            && m.destination == self.en_passant_square;
        return self.squares[m.destination as usize] != 0 || is_en_passant;
    }

    pub fn is_promotion(&self, m: &Move) -> bool {
        let moved_piece = self.squares[m.origin as usize];

        match moved_piece {
            WHITE_PAWN => m.destination >= 56,
            BLACK_PAWN => m.destination <= 7,
            _ => false,
        }
    }
}