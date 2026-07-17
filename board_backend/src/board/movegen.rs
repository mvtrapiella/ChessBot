// src/board/movegen.rs
use super::state::Board;
use super::types::{
    Move, WHITE_PAWN, WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, NO_SQUARE,
};

impl Board {
    pub fn move_generator(&self, origin: u8, piece: u8) -> Vec<Move> {
        let mut moves = Vec::new();

        match piece {
            WHITE_PAWN => {
                let column = origin % 8;

                // First move of the white pawn. It can be moved to squares forward
                if origin >= 8 && origin <= 15
                    && (self.all_pieces & (1_u64 << (origin + 8))) == 0 
                    && (self.all_pieces & (1_u64 << (origin + 16))) == 0 
                {
                    moves.push(Move { origin, destination: origin + 16, promotion: None });
                }

                // Movement without promotion
                if origin < 56 {
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
                        self.push_promotions(&mut moves, origin, origin + 8);
                    }

                    // Pawn on A column
                    if column == 0 {
                        // Capture to the right
                        if (self.black_pieces & (1_u64 << (origin + 9))) != 0 {
                            self.push_promotions(&mut moves, origin, origin + 9);
                        }
                    } 
                    // Pawn on H column
                    else if column == 7 {
                        if (self.black_pieces & (1_u64 << (origin + 7))) != 0 {
                            self.push_promotions(&mut moves, origin, origin + 7);
                        }
                    } 
                    // Rest of the pawns
                    else {
                        if (self.black_pieces & (1_u64 << (origin + 7))) != 0 {
                            self.push_promotions(&mut moves, origin, origin + 7);
                        }
                        if (self.black_pieces & (1_u64 << (origin + 9))) != 0 {
                            self.push_promotions(&mut moves, origin, origin + 9);
                        }
                    }
                }
            }
            _ => {}
        }

        moves
    }

    fn push_promotions(&self, moves: &mut Vec<Move>, origin: u8, destination: u8) {
        // Promote to a bishop
        moves.push(Move { origin, destination, promotion: Some(WHITE_BISHOP) });
        // Promote to a knight
        moves.push(Move { origin, destination, promotion: Some(WHITE_KNIGHT) });
        // Promote to a rook
        moves.push(Move { origin, destination, promotion: Some(WHITE_ROOK) });
        // Promote to a queen
        moves.push(Move { origin, destination, promotion: Some(WHITE_QUEEN) });
    }
}