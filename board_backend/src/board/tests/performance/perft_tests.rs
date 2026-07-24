use crate::board::position::Position;
use crate::board::state::Board;
use crate::board::types::{Color, NO_SQUARE};

fn starting_position() -> Position {
    let mut board = Board {
        squares: [0; 64],
        piece_bitboards: [0; 12],
        white_pieces: 0,
        black_pieces: 0,
        all_pieces: 0,
        side_to_move: Color::White,
        castling_rights: 15,
        en_passant_square: NO_SQUARE,
        halfmove_clock: 0,
    };
    board.initialize_board();
    board.update_bitboards();

    Position { board, history: Vec::new() }
}

// Reference values from the Chess Programming Wiki for the standard starting position.
#[test]
fn perft_matches_known_values_for_starting_position() {
    let mut pos = starting_position();

    assert_eq!(pos.perft(1), 20);
    assert_eq!(pos.perft(2), 400);
    assert_eq!(pos.perft(3), 8902);
    assert_eq!(pos.perft(4), 197281);
    assert_eq!(pos.perft(5), 4865609);
}
