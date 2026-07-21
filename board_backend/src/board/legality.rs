use crate::board::{Board, Color};
use super::types::{
    Move, EMPTY, WHITE_PAWN, WHITE_KING, BLACK_PAWN, BLACK_KING,
};

impl Board{
    pub fn legal_moves(&self, origin: u8, piece: u8) -> Vec<Move> {
        let mover_color = if piece <= 6 { Color::White } else { Color::Black };
        let pseudo_legal = self.move_generator(origin, piece);
        let is_king = piece == WHITE_KING || piece == BLACK_KING;

        let mut legal = Vec::new();
        let mut i = 0;
        while i < pseudo_legal.len() {
            let mv = pseudo_legal[i];

            // A king move that jumps 2 squares is a castle; movegen.rs emits it as this king
            // move immediately followed by the paired rook move. Validate both as one unit.
            if is_king && (mv.destination as i8 - mv.origin as i8).abs() == 2 {
                if self.castle_is_legal(mover_color, &mv) {
                    legal.push(mv);
                    legal.push(pseudo_legal[i + 1]);
                }
                i += 2;
            } else {
                if self.simulate_move(&mv, mover_color) {
                    legal.push(mv);
                }
                i += 1;
            }
        }

        legal
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