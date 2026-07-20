use super::common::{calculate_offsets, build_attacks_table};

// Magic number for each square calculated by executing the magic_numbers main.rs
pub const ROOK_MAGICS: [u64; 64] = [
  0x0080001020804000,  0x0040001000200048,  0x8200204200801009,  0x0100050008201002,
  0xa080028800808400,  0x0600020010080415,  0x1100040082000100,  0x0200108040240a01,
  0x0850800020400082,  0x0152002042108100,  0x800a001200244086,  0x0000801000800800,
  0x0001001008020500,  0x0000800400800200,  0x1104000402086110,  0x010a000200408104,
  0x0010218002884001,  0x0000808040002000,  0x0050808020001000,  0x8400210009001000,
  0x0108008004000880,  0x0084808004000200,  0x0c00040010080201,  0x4000020000a1004c,
  0x0001400180006080,  0x0520820200410021,  0x8020008080100020,  0x0010040040400800,
  0x2004000480080080,  0x0000020080040080,  0x8008108400010842,  0x0000800280014500,
  0x061040018c800422,  0x0000400084802010,  0x5410104101002000,  0xd010080080801000,
  0x8010040801001100,  0x2110800400800200,  0x1401000401000200,  0x0212004102002084,
  0x6000400080008020,  0x0030004820004000,  0x0110801200420022,  0x8000090010010021,
  0x0388000811010004,  0x0001000400090002,  0x04a0100108040002,  0x1040804400820001,
  0x1094214080090100,  0x1040002010080120,  0x0220801200204200,  0x0300100021000900,
  0x0242002008041200,  0x2000040002008080,  0xa030021108501400,  0x2001140081284200,
  0x0824800100496211,  0x0100810040002011,  0x040820000b005043,  0x1004040900201001,
  0x2083000800040211,  0x0009001400020803,  0x80e0008228100104,  0x8900044285003402,
];

// The shift is 64 - R_BITS[square]
pub const ROOK_SHIFTS: [u32; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53,
    52, 53, 53, 53, 53, 53, 53, 52,
];

// A mask that contains the possible movements of a rook from each square of the board
pub static ROOK_MASKS: [u64; 64] = calculate_all_rook_masks();

// With the magic numbers we make sure that there are no destructive collisions with the same blocking cases
// but it can be that another square magical number give as the same index so it overlaps the cases of another square.
// In order to fix that we add an offset that sums the number of cases of the previous square (first -> 0, second -> prev + 2^(prev bits) ...)
// By doing this is like we are "booking" a full floor with space so all the cases of each square are sure to not overlap the cases of the other squares
pub static ROOK_OFFSETS: [usize; 64] = calculate_offsets(&ROOK_SHIFTS);

// The precalculated 102400 possible rook movements.
// Computing this at compile time is legitimately slow work for the const evaluator (not a runaway
// loop), so the long_running_const_eval guard is explicitly waived here.
#[allow(long_running_const_eval)]
pub static ROOK_ATTACKS_TABLE: [u64; 102400] = calculate_all_rook_attacks();

const fn calculate_all_rook_masks() -> [u64; 64] {
    let mut masks = [0u64; 64];
    let mut square = 0;

    while square < 64 {
        let mut attacks = 0u64;
        // Row
        let target_r = (square / 8) as i32;
        // Column
        let target_f = (square % 8) as i32;

        // North
        let (mut r, f) = (target_r + 1, target_f);
        while r <= 6 { attacks |= 1u64 << (f + r * 8); r += 1; }

        // South
        let (mut r, f) = (target_r - 1, target_f);
        while r > 0 { attacks |= 1u64 << (f + r * 8); r -= 1; }

        // East
        let (r, mut f) = (target_r, target_f + 1);
        while f <= 6 { attacks |= 1u64 << (f + r * 8); f += 1; }

        // West
        let (r, mut f) = (target_r, target_f - 1);
        while f > 0 { attacks |= 1u64 << (f + r * 8); f -= 1; }

        masks[square] = attacks;
        square += 1;
    }

    masks
}

const fn rook_attacks_bruteforce(sq: i32, blockers: u64) -> u64 {
    let mut result = 0u64;
    // The row
    let target_r = sq / 8;
    // The column
    let target_f = sq % 8;

    // North
    let (mut r, f) = (target_r + 1, target_f);
    while r <= 7 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        r += 1;
    }

    // South
    let (mut r, f) = (target_r - 1, target_f);
    while r >= 0 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        r -= 1;
    }

    // East
    let (r, mut f) = (target_r, target_f + 1);
    while f <= 7 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        f += 1;
    }

    // West
    let (r, mut f) = (target_r, target_f - 1);
    while f >= 0 {
        let bit = 1u64 << (f + r * 8);
        result |= bit;
        if (blockers & bit) != 0 { break; }
        f -= 1;
    }

    result
}

const fn calculate_all_rook_attacks() -> [u64; 102400] {
    build_attacks_table!(102400, ROOK_MASKS, ROOK_MAGICS, ROOK_SHIFTS, ROOK_OFFSETS, rook_attacks_bruteforce)
}
