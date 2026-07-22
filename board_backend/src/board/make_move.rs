use crate::board::{position::Position, types::{
    BLACK_KING, BLACK_PAWN, BLACK_ROOK, EMPTY, Move, NO_SQUARE, WHITE_KING, WHITE_PAWN, WHITE_ROOK,
}};

pub struct Action {
    pub mv: Move,
    pub captured_piece: Option<u8>,
    pub captured_square: Option<u8>,
    pub moved_piece: u8,
    pub previous_castling_rights: u8,
    pub previous_en_passant_square: u8,
    pub previous_halfmove_clock: u8,
}

const WHITE_SHORT_CASTLE: u8 = 0x01;
const WHITE_LONG_CASTLE: u8 = 0x02;
const BLACK_SHORT_CASTLE: u8 = 0x04;
const BLACK_LONG_CASTLE: u8 = 0x08;

impl Position {
    pub fn make_move(&mut self, m: Move) {
        let moved_piece = self.board.squares[m.origin as usize];
        let is_pawn = moved_piece == WHITE_PAWN || moved_piece == BLACK_PAWN;

        let is_en_passant = is_pawn
            && self.board.squares[m.destination as usize] == EMPTY
            && m.destination == self.board.en_passant_square;

        let (captured_piece, captured_square) = if is_en_passant {
            let captured_sq = if moved_piece == WHITE_PAWN { m.destination - 8 } else { m.destination + 8 };
            (Some(self.board.squares[captured_sq as usize]), Some(captured_sq))
        } else if self.board.squares[m.destination as usize] != EMPTY {
            (Some(self.board.squares[m.destination as usize]), Some(m.destination))
        } else {
            (None, None)
        };

        let action = Action {
            mv: m,
            captured_piece,
            captured_square,
            moved_piece,
            previous_castling_rights: self.board.castling_rights,
            previous_en_passant_square: self.board.en_passant_square,
            previous_halfmove_clock: self.board.halfmove_clock,
        };

        // --- Mutate squares[] ---
        if is_en_passant {
            self.board.squares[captured_square.expect("en passant always has a captured square") as usize] = EMPTY;
        }

        // If this is a castle, the king jumps 2 squares; the rook's own move is derived
        // from the king's destination rather than tracked as a separate Move.
        let mut rook_move: Option<(u8, u8)> = None; // (rook_origin, rook_destination)
        if moved_piece == WHITE_KING && (m.origin as i8 - m.destination as i8).abs() == 2 {
            rook_move = Some(match m.destination {
                2 => (0, 3),
                6 => (7, 5),
                _ => panic!("white king jump of 2 squares to an unknown castle destination: {}", m.destination),
            });
        } else if moved_piece == BLACK_KING && (m.origin as i8 - m.destination as i8).abs() == 2 {
            rook_move = Some(match m.destination {
                58 => (56, 59),
                62 => (63, 61),
                _ => panic!("black king jump of 2 squares to an unknown castle destination: {}", m.destination),
            });
        }
        if let Some((rook_origin, rook_destination)) = rook_move {
            self.board.squares[rook_destination as usize] = self.board.squares[rook_origin as usize];
            self.board.squares[rook_origin as usize] = EMPTY;
        }

        let placed_piece = m.promotion.unwrap_or(moved_piece);
        self.board.squares[m.destination as usize] = placed_piece;
        self.board.squares[m.origin as usize] = EMPTY;

        // --- Castling rights: king move loses both of that color's rights, a rook
        // moving off (or being captured on) its home square loses that one right. ---
        match moved_piece {
            WHITE_KING => self.board.castling_rights &= !(WHITE_SHORT_CASTLE | WHITE_LONG_CASTLE),
            BLACK_KING => self.board.castling_rights &= !(BLACK_SHORT_CASTLE | BLACK_LONG_CASTLE),
            WHITE_ROOK if m.origin == 0 => self.board.castling_rights &= !WHITE_LONG_CASTLE,
            WHITE_ROOK if m.origin == 7 => self.board.castling_rights &= !WHITE_SHORT_CASTLE,
            BLACK_ROOK if m.origin == 56 => self.board.castling_rights &= !BLACK_LONG_CASTLE,
            BLACK_ROOK if m.origin == 63 => self.board.castling_rights &= !BLACK_SHORT_CASTLE,
            _ => {}
        }
        if let (Some(captured), Some(csq)) = (captured_piece, captured_square) {
            match (captured, csq) {
                (WHITE_ROOK, 0) => self.board.castling_rights &= !WHITE_LONG_CASTLE,
                (WHITE_ROOK, 7) => self.board.castling_rights &= !WHITE_SHORT_CASTLE,
                (BLACK_ROOK, 56) => self.board.castling_rights &= !BLACK_LONG_CASTLE,
                (BLACK_ROOK, 63) => self.board.castling_rights &= !BLACK_SHORT_CASTLE,
                _ => {}
            }
        }

        // --- En passant square: reset every move, except set it on a pawn double-push. ---
        let is_double_push = is_pawn && (m.origin as i16 - m.destination as i16).abs() == 16;
        self.board.en_passant_square = if is_double_push {
            (m.origin + m.destination) / 2
        } else {
            NO_SQUARE
        };

        // --- Halfmove clock: reset on a pawn move or a capture, otherwise increment. ---
        if captured_piece.is_some() || is_pawn {
            self.board.reset_clock();
        } else {
            self.board.increase_clock();
        }

        // --- Piece bitboards: each entry is exactly one piece type, so these toggles are
        // unambiguous regardless of capture/en passant/castling/promotion. ---
        let origin_mask = 1u64 << m.origin;
        let destination_mask = 1u64 << m.destination;

        if placed_piece == moved_piece {
            self.board.piece_bitboards[moved_piece as usize - 1] ^= origin_mask | destination_mask;
        } else {
            // Promotion: the pawn disappears from origin, the new piece appears at destination.
            self.board.piece_bitboards[moved_piece as usize - 1] ^= origin_mask;
            self.board.piece_bitboards[placed_piece as usize - 1] ^= destination_mask;
        }

        if let Some((rook_origin, rook_destination)) = rook_move {
            let rook_piece = self.board.squares[rook_destination as usize];
            self.board.piece_bitboards[rook_piece as usize - 1] ^= (1u64 << rook_origin) | (1u64 << rook_destination);
        }

        if let (Some(captured), Some(csq)) = (captured_piece, captured_square) {
            self.board.piece_bitboards[captured as usize - 1] ^= 1u64 << csq;
        }

        // --- Aggregates: recompute from the now-correct piece bitboards, rather than
        // hand-toggling white_pieces/black_pieces/all_pieces. A normal capture's destination
        // square is occupied both before and after the move (just by different pieces), so
        // there's no clean universal "does this bit flip" rule across quiet/capture/en
        // passant/castling/promotion — recomputing sidesteps that ambiguity entirely. ---

        // fold is the equivalent to reduce in other languages and |acc, bb| acc | bb is the equivalent to: (acc, bb) => acc | bb a lamda expression
        self.board.white_pieces = self.board.piece_bitboards[0..6].iter().fold(0u64, |acc, bb| acc | bb);
        self.board.black_pieces = self.board.piece_bitboards[6..12].iter().fold(0u64, |acc, bb| acc | bb);
        self.board.all_pieces = self.board.white_pieces | self.board.black_pieces;

        self.history.push(action);
        self.board.switch_turn();
    }
}
