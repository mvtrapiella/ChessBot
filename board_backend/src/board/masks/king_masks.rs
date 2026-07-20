const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;

const NOT_1_ROW: u64 = 0xFFFFFFFFFFFFFF00; // 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 0000 0000

const NOT_8_ROW: u64 = 0x00FFFFFFFFFFFFFF; // 0000 0000 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111 1111

const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;

pub const KING_ATTACKS: [u64; 64] = calculate_king_attacks();

// Function that generates a table with all the precomputed movements of the horse
const fn calculate_king_attacks() -> [u64; 64] {
    // Initially is full of 0
    let mut attacks = [0u64; 64];
    // First square (A0)
    let mut square = 0;
    
    while square < 64 {
        let bitboard = 1u64 << square;
        let mut king_bitboard = 0u64;

        // Up - middel
        if (bitboard << 8) & NOT_1_ROW != 0 { king_bitboard |= bitboard << 8; }

        // Up - left
        if (bitboard << 7) & NOT_1_ROW & NOT_H_FILE != 0 { king_bitboard |= bitboard << 7; }

        // Up - right
        if (bitboard << 9) & NOT_1_ROW & NOT_A_FILE != 0 { king_bitboard |= bitboard << 9; }

        // Middle - right
        if (bitboard << 1) & NOT_A_FILE != 0 { king_bitboard |= bitboard << 1; }

        // Middle - left
        if (bitboard >> 1) & NOT_H_FILE != 0 { king_bitboard |= bitboard >> 1; }

        // Down - left
        if (bitboard >> 9) & NOT_8_ROW & NOT_H_FILE != 0 { king_bitboard |= bitboard >> 9; }

        //  Down - middle
        if (bitboard >> 8) & NOT_8_ROW != 0 { king_bitboard |= bitboard >> 8; }

        // Down - right
        if (bitboard >> 7) & NOT_8_ROW & NOT_A_FILE != 0 { king_bitboard |= bitboard >> 7; }


        attacks[square] = king_bitboard;
        square += 1;
    }
    
    attacks
}
