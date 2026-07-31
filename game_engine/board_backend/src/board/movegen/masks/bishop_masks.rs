use super::common::{calculate_offsets, build_attacks_table};

// Magic number for each square calculated by executing the magic_numbers main.rs
pub const BISHOP_MAGICS: [u64; 64] = [
  0x40022202041c0080,  0x0004100092008240,  0x0022480040824040,  0x4402408d00100880,
  0x0551104002008042,  0x0000882008406000,  0x000200b004100000,  0x4021120809043000,
  0x8000101082a80040,  0x2080090208005900,  0x01980802006a0000,  0x0002041046001000,
  0x6000220210100000,  0xb040220804060000,  0x4000444a02104020,  0x4060010101012004,
  0x082040302081810c,  0x001100080200a400,  0x1002000420240100,  0x080104c8204200b0,
  0x0042104401041184,  0xb882008100824100,  0x0005209404880c00,  0x0110243a8a841000,
  0x2022c00820050400,  0x0010220108180104,  0x0010490008080900,  0x8520080016081050,
  0x0182040162008200,  0x0050808009082000,  0x1028288081008800,  0x0a10404111040200,
  0x180108a000082080,  0x0e42300200040801,  0x8186003001020880,  0x6002028180080200,
  0x2108002400124100,  0x0008020021041000,  0x80040c1404006110,  0x0005210900020060,
  0x0402016020025a00,  0x8000841002002880,  0x100208a488001010,  0x9040014208000080,
  0x010a24104400ca88,  0x0024200404093040,  0x1210010800940101,  0x808280820200808a,
  0x2190840420045000,  0x0000240108080005,  0x0400804220900084,  0x0880018220a80070,
  0x0012a04010410404,  0x2000c06208024004,  0x1424280801341000,  0x0020844080910208,
  0x0084110098044000,  0x010000310110100a,  0x8000408284088808,  0x82a001d004420202,
  0x0008025c04050c02,  0x2004004050026080,  0x001358885014105a,  0x40a0211202040820,
];

// The shift is the result of 64 - square bits (64 - 6, 64 -5 ...)
// We could also store the bits and in the shifting operation do the substraction (6, 5, 5 ...)
pub const BISHOP_SHIFTS: [u32; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58,
    59, 59, 59, 59, 59, 59, 59, 59,
    59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59,
    59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59,
    58, 59, 59, 59, 59, 59, 59, 58
];

// A mask that contains the possible movements of a bishop from each square of the board
pub static BISHOP_MASKS: [u64; 64] = calculate_all_bishop_masks();

// With the magic numbers we make sure that there are no destructive collisions with the same blocking cases
// but it can be that another square magical number give as the same index so it overlaps the cases of another square.
// In order to fix that we add an offset that sums the number of cases of the previous square (first -> 0, second -> prev + 2^(prev bits) = 0 + 2^6 = 64...)
// By doing this is like we are "booking" a full floor with space so all the cases of each square are sure to not overlap the cases of the other squares
pub static BISHOP_OFFSETS: [usize; 64] = calculate_offsets(&BISHOP_SHIFTS);

// The precalculated 5248 possible bishop movements
pub static BISHOP_ATTACKS_TABLE: [u64; 5248] = calculate_all_bishop_attacks();

const fn calculate_all_bishop_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let mut attacks = 0u64;
        // Row
        let r = (square / 8) as i32;
        // Column
        let f = (square % 8) as i32;

        // North East
        let (mut rank, mut file) = (r + 1, f + 1);
        while rank < 7 && file < 7 {
            attacks |= 1u64 << (rank * 8 + file);
            rank += 1; file += 1;
        }

        // North West
        let (mut rank, mut file) = (r + 1, f - 1);
        while rank < 7 && file > 0 {
            attacks |= 1u64 << (rank * 8 + file);
            rank += 1; file -= 1;
        }

        // South East
        let (mut rank, mut file) = (r - 1, f + 1);
        while rank > 0 && file < 7 {
            attacks |= 1u64 << (rank * 8 + file);
            rank -= 1; file += 1;
        }

        // South West
        let (mut rank, mut file) = (r - 1, f - 1);
        while rank > 0 && file > 0 {
            attacks |= 1u64 << (rank * 8 + file);
            rank -= 1; file -= 1;
        }

        masks[square] = attacks;
        square += 1;
    }

    masks
}

const fn bishop_attacks_bruteforce(sq: i32, blockers: u64) -> u64 {
    // The obstacle bitboard initially all 0
    let mut result = 0u64;
    // The row
    let target_r = sq / 8;
    // The column
    let target_f = sq % 8;

    // North east
    let (mut r, mut f) = (target_r + 1, target_f + 1);
    // Check that the square is inside the board (up and right limit)
    while r <= 7 && f <= 7 {
        // Calculate the cell of the board
        let bit = 1u64 << (f + r * 8);
        // We add the movement before checking if it colides because if it colides we just stop the loop
        // That collision can be caused by a enemy piece in which case we could capture or an ally piece in which case we could not move
        // This is cheecked in another phase with a & ! with the white_pieces
        result |= bit;
        if (blockers & bit) != 0 { break; }
        r += 1; f += 1;
    }

    // North west
    let (mut r, mut f) = (target_r + 1, target_f - 1);
    while r <= 7 && f >= 0 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        r += 1; f -= 1;
    }

    // South east
    let (mut r, mut f) = (target_r - 1, target_f + 1);
    while r >= 0 && f <= 7 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        r -= 1; f += 1;
    }

    // South west
    let (mut r, mut f) = (target_r - 1, target_f - 1);
    while r >= 0 && f >= 0 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        r -= 1; f -= 1;
    }

    result
}

const fn calculate_all_bishop_attacks() -> [u64; 5248] {
    build_attacks_table!(5248, BISHOP_MASKS, BISHOP_MAGICS, BISHOP_SHIFTS, BISHOP_OFFSETS, bishop_attacks_bruteforce)
}
