use std::fs;
use std::io;
use std::path::Path;
use std::sync::OnceLock;

use crate::board::polyglot_hash::polyglot_hash;
use crate::board::state::Board;
use crate::board::types::{
    Color, Move, WHITE_KNIGHT, WHITE_BISHOP, WHITE_ROOK, WHITE_QUEEN,
    BLACK_KNIGHT, BLACK_BISHOP, BLACK_ROOK, BLACK_QUEEN,
};

const RECORD_SIZE: usize = 16;
// Plies, not full moves -- 20 plies is 10 moves each side. Book coverage naturally
// runs out before this in most lines anyway; this is just a backstop.
const MAX_BOOK_PLIES: u32 = 20;

// One raw entry as stored in a Polyglot .bin book, transcribed with minimal
// interpretation. `promotion` is kept as Polyglot's own raw code rather than
// resolved into one of this engine's colored piece codes (WHITE_QUEEN vs
// BLACK_QUEEN, etc.) -- that resolution needs to know whose move it is, which
// isn't part of the book file itself, only of the position being looked up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BookEntry {
    pub key: u64,
    pub origin: u8,
    pub destination: u8,
    // Polyglot's own code: 0 = none, 1 = knight, 2 = bishop, 3 = rook, 4 = queen.
    pub promotion: u8,
    pub weight: u16,
}

#[derive(Debug)]
pub enum OpeningBookError {
    Io(io::Error),
    // The file's length isn't a multiple of 16 bytes, so it can't be a valid
    // sequence of Polyglot records.
    MalformedLength(usize),
}

impl From<io::Error> for OpeningBookError {
    fn from(err: io::Error) -> Self {
        OpeningBookError::Io(err)
    }
}

pub fn load_book(path: impl AsRef<Path>) -> Result<Vec<BookEntry>, OpeningBookError> {
    let bytes = fs::read(path)?;
    parse_book(&bytes)
}

pub fn parse_book(bytes: &[u8]) -> Result<Vec<BookEntry>, OpeningBookError> {
    if bytes.len() % RECORD_SIZE != 0 {
        return Err(OpeningBookError::MalformedLength(bytes.len()));
    }

    Ok(bytes.chunks_exact(RECORD_SIZE).map(parse_record).collect())
}

// Polyglot records are big-endian: 8 bytes key, 2 bytes move, 2 bytes weight,
// 2 bytes "learn" (historically engine-specific, unused here).
fn parse_record(record: &[u8]) -> BookEntry {
    let key = u64::from_be_bytes(record[0..8].try_into().unwrap());
    let mv = u16::from_be_bytes(record[8..10].try_into().unwrap());
    let weight = u16::from_be_bytes(record[10..12].try_into().unwrap());

    // Bit layout, low to high: to-file(3) to-rank(3) from-file(3) from-rank(3) promotion(3) unused(1).
    let to_file = mv & 0b111;
    let to_rank = (mv >> 3) & 0b111;
    let from_file = (mv >> 6) & 0b111;
    let from_rank = (mv >> 9) & 0b111;
    let promotion = ((mv >> 12) & 0b111) as u8;

    // Matches this engine's own square indexing (a1 = 0 .. h8 = 63, index = rank*8 + file),
    // since Polyglot's file/rank numbering (0 = a/rank1) already lines up with it.
    let origin = (from_rank * 8 + from_file) as u8;
    let destination = (to_rank * 8 + to_file) as u8;

    BookEntry { key, origin, destination, promotion, weight }
}

static BOOK: OnceLock<Vec<BookEntry>> = OnceLock::new();

// Embedded at compile time -- no runtime file path to configure or ship separately,
// and the Docker build already copies this whole crate directory into its build context.
fn loaded_book() -> &'static [BookEntry] {
    BOOK.get_or_init(|| {
        let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/books/gm2001.bin"));
        parse_book(bytes).expect("embedded gm2001.bin should be a well-formed Polyglot book")
    })
}

// The highest-weighted known reply to the current position, if the book covers it and
// we're still within the opening. Always re-validated against the position's own actual
// legal moves before being trusted -- a hash match alone doesn't rule out a collision.
pub fn book_move(board: &Board, moves_counter: u32) -> Option<Move> {
    if moves_counter >= MAX_BOOK_PLIES {
        return None;
    }

    let hash = polyglot_hash(board);
    let best_entry = loaded_book()
        .iter()
        .filter(|entry| entry.key == hash)
        .max_by_key(|entry| entry.weight)?;

    let mv = to_move(best_entry, board.side_to_move)?;

    board.all_legal_moves().contains(&mv).then_some(mv)
}

// pub(crate) rather than private so tests can exercise the castling remap directly with
// constructed entries, instead of depending on the embedded book happening to recommend
// a castle for some reachable position.
pub(crate) fn to_move(entry: &BookEntry, side_to_move: Color) -> Option<Move> {
    let promotion = match entry.promotion {
        0 => None,
        1 => Some(if side_to_move == Color::White { WHITE_KNIGHT } else { BLACK_KNIGHT }),
        2 => Some(if side_to_move == Color::White { WHITE_BISHOP } else { BLACK_BISHOP }),
        3 => Some(if side_to_move == Color::White { WHITE_ROOK } else { BLACK_ROOK }),
        4 => Some(if side_to_move == Color::White { WHITE_QUEEN } else { BLACK_QUEEN }),
        _ => return None,
    };

    let destination = remap_castling_destination(entry.origin, entry.destination);

    Some(Move { origin: entry.origin, destination, promotion })
}

// Polyglot encodes castling as the king "capturing" its own rook (e.g. white kingside
// castle is stored as e1h1) rather than the king moving straight to g1/c1/g8/c8 like this
// engine represents it. These four origin/destination pairs are unambiguous: no other
// piece can ever legally move from a king's home square straight onto its own rook's home
// square, so remapping them unconditionally (without checking whose move it is or what's
// actually on the board) is safe -- and if castling isn't actually legal in this position
// for any reason, the remapped move still has to pass the caller's all_legal_moves() check.
fn remap_castling_destination(origin: u8, destination: u8) -> u8 {
    match (origin, destination) {
        (4, 7) => 6,    // white kingside: e1h1 -> e1g1
        (4, 0) => 2,    // white queenside: e1a1 -> e1c1
        (60, 63) => 62, // black kingside: e8h8 -> e8g8
        (60, 56) => 58, // black queenside: e8a8 -> e8c8
        _ => destination,
    }
}
