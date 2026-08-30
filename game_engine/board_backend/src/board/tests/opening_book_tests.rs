use crate::board::opening_book::load_book;

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
