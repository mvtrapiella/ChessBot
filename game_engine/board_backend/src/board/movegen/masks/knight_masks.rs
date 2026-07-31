// Masks that prevent than a horse in the left columns (A or B) can go overflow (to the other side of the board -> illegal)
// and that the horses in the right columns (G or H) cannot go overflow to the left side of the board
const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE; // -> 1111 1110 1111 1110 1111 1110 1111 1110 1111 1110 1111 1110 1111 1110 1111 1110
// Because of how is the bitboard the least significatgibe bit (the right most) is the first square (0, 0) and the left most the last (7, 7).
// So in order to visualize the board we have to traverse the bit sequence backwards
/* Mask that ensures that all the horses except the ones at the 
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
0   1   1   1   1   1   1   1
*/
const NOT_B_FILE: u64 = 0xFDFDFDFDFDFDFDFD;  // -> 1111 1101 1111 1101 1111 1101 1111 1101 1111 1101 1111 1101 1111 1101 1111 1101
/*
1   0   1   1   1   1   1   1
1   0   1   1   1   1   1   1 
1   0   1   1   1   1   1   1 
1   0   1   1   1   1   1   1 
1   0   1   1   1   1   1   1 
1   0   1   1   1   1   1   1 
1   0   1   1   1   1   1   1 
1   0   1   1   1   1   1   1  
*/
const NOT_G_FILE: u64 = 0xBFBFBFBFBFBFBFBF; // -> 1011 1111 1011 1111 1011 1111 1011 1111 1011 1111 1011 1111 1011 1111 1011 1111
/*
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
1   1   1   1   1   1   0   1
*/
const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F; // -> 0111 1111 0111 11110111 11110111 11110111 11110111 11110111 11110111 1111
/*
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
1   1   1   1   1   1   1   0
*/

// Static matrix with all the possible movements of the knight. The table is compiled one time at the beginning and then
// we can access to all the possible positions of the knight instantly
pub const KNIGHT_ATTACKS: [u64; 64] = calculate_knight_attacks();

// Function that generates a table with all the precomputed movements of the horse
const fn calculate_knight_attacks() -> [u64; 64] {
    // Initially is full of 0
    let mut attacks = [0u64; 64];
    // First square (A0)
    let mut square = 0;
    
    while square < 64 {
        let bitboard = 1u64 << square;
        let mut knight_bitboard = 0u64;

        // In order to visualize bette the possible positions of the knight and the number of bits we have to swift: https://www.chessprogramming.org/Knight_Pattern
        //Jumps up and sides
        // Up right
        // We check that the resulting bitboard after the movement do not end on the A column (it would be impossible unless it is due to an overflow)
        // If you are jumping two squares up and one to the right, it is imposible to land on the A column (due to that square to the right, you coulD land on the B column if you are on the A one)
        // but it is impossible to land on the A column
        if (bitboard << 17) & NOT_A_FILE != 0 { knight_bitboard |= bitboard << 17; }

        // Up left
        // We check that the resulting bitboard after the movement do not end on the H column (it would be impossible unless it is due to an overflow)
        // If you are jumping two squares up and one to the left, it is imposible to land on the H column (due to that square to the left, you could land on the G column if you are on the H one)
        // but it is impossible to land on the H column
        if (bitboard << 15) & NOT_H_FILE != 0 { knight_bitboard |= bitboard << 15; }

        // Middle up right
        // We check that the resulting bitboard after the movement do not end on the A nor B column (it would be impossible unless it is due to an overflow)
        // If you are jumping one square up and two to the right, it is imposible to land on the A or B columns (due to that two squares to the right)
        if (bitboard << 10) & NOT_A_FILE & NOT_B_FILE != 0 { knight_bitboard |= bitboard << 10; }

        // Middle up left
        // We check that the resulting bitboard after the movement do not end on the H nor G column (it would be impossible unless it is due to an overflow)
        // If you are jumping one square up and two to the left, it is imposible to land on the H or G columns (due to that two squares to the left)
        if (bitboard << 6) & NOT_G_FILE & NOT_H_FILE != 0 { knight_bitboard |= bitboard << 6; }

        // Jumps down and sides
        // Down left
        if (bitboard >> 17) & NOT_H_FILE != 0 { knight_bitboard |= bitboard >> 17; }
        //  Down right
        if (bitboard >> 15) & NOT_A_FILE != 0 { knight_bitboard |= bitboard >> 15; }
        // Middle down left
        if (bitboard >> 10) & NOT_G_FILE & NOT_H_FILE != 0 { knight_bitboard |= bitboard >> 10; }
        // Middle down right
        if (bitboard >> 6) & NOT_A_FILE & NOT_B_FILE != 0 { knight_bitboard |= bitboard >> 6; }

        attacks[square] = knight_bitboard;
        square += 1;
    }
    
    attacks
}
