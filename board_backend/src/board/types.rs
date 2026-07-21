#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub const EMPTY: u8 = 0;
pub const WHITE_PAWN: u8 = 1;
pub const WHITE_ROOK: u8 = 2;
pub const WHITE_KNIGHT: u8 = 3;
pub const WHITE_BISHOP: u8 = 4;
pub const WHITE_QUEEN: u8 = 5;
pub const WHITE_KING: u8 = 6;
pub const BLACK_PAWN: u8 = 7;
pub const BLACK_ROOK: u8 = 8;
pub const BLACK_KNIGHT: u8 = 9;
pub const BLACK_BISHOP: u8 = 10;
pub const BLACK_QUEEN: u8 = 11;
pub const BLACK_KING: u8 = 12;

pub const NO_SQUARE: u8 = 64;

pub struct Move{
    pub origin: u8,
    pub destination: u8,
    pub promotion: Option<u8>,
}

