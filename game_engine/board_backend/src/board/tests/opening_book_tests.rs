use crate::board::opening_book::{load_book, to_move, BookEntry};
use crate::board::types::{Color, Move};

// CARGO_MANIFEST_DIR anchors this to the crate root regardless of the working
// directory tests happen to run from.
const GM2001_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/books/gm2001.bin");

#[test]
fn parses_the_real_gm2001_book_file() {
    let entries = load_book(GM2001_PATH).expect("gm2001.bin should parse as a well-formed Polyglot book");

    // The file is exactly 30416 sixteen-byte records (486656 bytes / 16).
    assert_eq!(entries.len(), 30416);

    // Every entry should have a plausible weight (real book data isn't all zeroes)
    // and squares within the board -- structurally guaranteed by the 3-bit masks
    // in parse_record, but worth asserting as a regression guard.
    assert!(entries.iter().any(|entry| entry.weight > 0));
    assert!(entries.iter().all(|entry| entry.origin < 64 && entry.destination < 64));

    // Polyglot's promotion code only ever ranges 0 (none) to 4 (queen).
    assert!(entries.iter().all(|entry| entry.promotion <= 4));
}

#[test]
fn remaps_all_four_polyglot_castling_encodings() {
    // (origin, Polyglot-encoded destination, side to move, this engine's own destination)
    let cases = [
        (4u8, 7u8, Color::White, 6u8),  // white kingside: e1h1 -> e1g1
        (4, 0, Color::White, 2),        // white queenside: e1a1 -> e1c1
        (60, 63, Color::Black, 62),     // black kingside: e8h8 -> e8g8
        (60, 56, Color::Black, 58),     // black queenside: e8a8 -> e8c8
    ];

    for (origin, destination, side_to_move, expected_destination) in cases {
        let entry = BookEntry { key: 0, origin, destination, promotion: 0, weight: 1 };
        let mv = to_move(&entry, side_to_move).expect("promotion 0 always converts");

        assert_eq!(mv, Move { origin, destination: expected_destination, promotion: None });
    }
}

#[test]
fn leaves_ordinary_moves_unaffected_by_the_castling_remap() {
    // e2e4 -- nowhere near any of the four castling-specific origin/destination pairs.
    let entry = BookEntry { key: 0, origin: 12, destination: 28, promotion: 0, weight: 1 };
    let mv = to_move(&entry, Color::White).expect("promotion 0 always converts");

    assert_eq!(mv, Move { origin: 12, destination: 28, promotion: None });
}
