use crate::board::{Board, Color};
use super::types::{
    Move, EMPTY, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, NO_SQUARE,
};

impl Board{
    pub fn legal_moves(&self, origin: u8, piece: u8) -> Vec<Move> {
        let mover_color = if piece <= 6 { Color::White } else { Color::Black };
        let pseudo_legal = self.move_generator(origin, piece);

        pseudo_legal.into_iter().filter(|mv| self.simulate_move(mv, mover_color)).collect()
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