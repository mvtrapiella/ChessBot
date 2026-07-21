// Shared, piece-agnostic helpers for building magic-bitboard attack tables.
// Used by both bishop_masks.rs and rook_masks.rs.

pub const fn pop_lsb(bb: &mut u64) -> i32 {
    // If the number is already 0
    if *bb == 0 { return -1; }
    // This function counts the number of 0 at the right of the LSO, which is equivalent to the position index of the LSO
    let lsb = bb.trailing_zeros() as i32;
    // We use Brian Kernighan trick -> "turn off" the least significant one
    *bb &= *bb - 1;
    lsb
}

pub const fn set_blockers(index: i32, mut mask: u64) -> u64 {
    let mut blockers = 0u64;
    let bits = mask.count_ones() as i32;

    let mut i = 0;
    while i < bits {
        let bit_index = pop_lsb(&mut mask);
        if (index & (1 << i)) != 0 {
            blockers |= 1u64 << bit_index;
        }
        i += 1;
    }
    blockers
}

// Cumulative "booking" offset for each square's slice of a shared magic-bitboard attack table:
// offset[0] = 0, offset[i+1] = offset[i] + 2^(64 - shifts[i]). Deriving this from the shift table
// instead of hand-transcribing it keeps the two arrays from ever drifting out of sync.
pub const fn calculate_offsets(shifts: &[u32; 64]) -> [usize; 64] {
    let mut offsets = [0usize; 64];
    let mut square = 0;
    let mut cumulative = 0usize;

    while square < 64 {
        offsets[square] = cumulative;
        let bits = 64 - shifts[square];
        cumulative += 1usize << bits;
        square += 1;
    }

    offsets
}

// Fills a magic-bitboard attack table for a sliding piece: for every square and every possible
// occupancy of its relevant-mask squares, hashes the blockers through that square's magic number
// and stores the real attack bitboard at offset + hash. $bruteforce is the piece-specific function
// that turns (square, blockers) into the real attack bitboard.
macro_rules! build_attacks_table {
    ($table_size:expr, $masks:expr, $magics:expr, $shifts:expr, $offsets:expr, $bruteforce:ident) => {{
        let mut table = [0u64; $table_size];
        let mut square = 0;

        while square < 64 {
            let mask = $masks[square];
            let num_bits = mask.count_ones();
            let num_combinations = 1u32 << num_bits;

            let magic = $magics[square];
            let shift = $shifts[square];
            let offset = $offsets[square];

            let mut i = 0u32;
            while i < num_combinations {
                let blockers = crate::board::masks::common::set_blockers(i as i32, mask);
                let real_attack = $bruteforce(square as i32, blockers);
                let hash = (blockers.wrapping_mul(magic) >> shift) as usize;

                table[offset + hash] = real_attack;

                i += 1;
            }

            square += 1;
        }

        table
    }};
}

pub(crate) use build_attacks_table;
