use crate::board::{Board, Color};
use super::types::{
    Move, EMPTY, WHITE_PAWN, WHITE_KING, BLACK_PAWN, BLACK_KING,
};

impl Board{
    pub fn legal_moves(&self, origin: u8, piece: u8) -> Vec<Move> {
        let mover_color = if piece <= 6 { Color::White } else { Color::Black };
        let pseudo_legal = self.move_generator(origin, piece);
        let is_king = piece == WHITE_KING || piece == BLACK_KING;

        // A king move that jumps 2 squares is a castle; castle_is_legal checks the king's
        // path against the current board instead of simulating (apply_move_bits already
        // derives and moves the rook itself from the king's destination square).
        pseudo_legal.into_iter().filter(|mv| {
            if is_king && (mv.destination as i8 - mv.origin as i8).abs() == 2 {
                self.castle_is_legal(mover_color, mv)
            } else {
                self.simulate_move(mv, mover_color)
            }
        }).collect()
    }

    // Castling doesn't need simulate_move's clone-and-check: the intervening squares are
    // already guaranteed empty by movegen.rs's castling emptiness masks, so the only thing
    // left to verify is that the king isn't currently in check, doesn't pass through an
    // attacked square, and doesn't land on one.
    fn castle_is_legal(&self, mover_color: Color, king_move: &Move) -> bool {
        let enemy = mover_color.opposite();
        let transit_square = (king_move.origin + king_move.destination) / 2;

        !self.is_square_attacked(king_move.origin, enemy)
            && !self.is_square_attacked(transit_square, enemy)
            && !self.is_square_attacked(king_move.destination, enemy)
    }


    fn apply_move_bits(&mut self, mv: &Move, mover_color: Color) {
        let piece = self.squares[mv.origin as usize];

        // En passant: the captured pawn sits behind the destination square, not on it.
        let is_pawn = piece == WHITE_PAWN || piece == BLACK_PAWN;
        if is_pawn && mv.destination == self.en_passant_square && self.squares[mv.destination as usize] == EMPTY {
            let captured_square = match mover_color {
                Color::White => mv.destination - 8,
                Color::Black => mv.destination + 8,
            };
            self.squares[captured_square as usize] = EMPTY;
        }

        // Castling: also move the rook to its castled square. The king's destination
        // uniquely identifies which castle this is, matching the squares movegen.rs
        // already hardcodes for each of the four castling emissions.
        let is_king = piece == WHITE_KING || piece == BLACK_KING;
        if is_king && (mv.destination as i8 - mv.origin as i8).abs() == 2 {
            let (rook_origin, rook_destination) = match mv.destination {
                6 => (7, 5),
                2 => (0, 3),
                62 => (63, 61),
                58 => (56, 59),
                _ => unreachable!("king move of 2 squares that isn't a known castle destination"),
            };
            self.squares[rook_destination as usize] = self.squares[rook_origin as usize];
            self.squares[rook_origin as usize] = EMPTY;
        }

        self.squares[mv.destination as usize] = mv.promotion.unwrap_or(piece);
        self.squares[mv.origin as usize] = EMPTY;
    }

    pub fn simulate_move(&self, mv: &Move, mover_color: Color) -> bool {
        let mut clone = *self;
        clone.apply_move_bits(mv, mover_color);
        clone.update_bitboards();

        !clone.is_in_check(mover_color)
    }
}