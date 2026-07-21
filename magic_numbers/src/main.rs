use std::num::Wrapping;

// Pseudo random number generator
struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    // Constructor
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    // Implements the random algorithm by George Marsaglia on 2003
    // It basically uses XOR in order to swift the bits dramatically and do some bits shifts that are
    // demonstrated using groups theory that for every seed (from 0 to 64) except 0 there are 2^64 - 1 possible numbers
    // before repeating a number
    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    // The possibility of  number being 1 is 50 % (1 or 0). If we apply the & operator (like normal number *)
    // The probability goes (1/2)^2 = 25%. If we do another AND the probability is (1/2)^3 = 12.5 % -> so most bits will be 0
    fn few_bits(&mut self) -> u64 {
        self.next() & self.next() & self.next()
    }
}

// Count the number of 1 bits on the number
fn count_bits(mut b: u64) -> u32 {
    let mut count = 0;
    // When the number is 0 there are no more 1 (0000 0000 ... 0000)
    while b != 0 {
        count += 1;
        b &= b - 1; // Brian Kernighan trick -> "turn off" the least significant one
    }
    count
}

// Returns the index of the LSO and it turns it off
fn pop_lsb(bb: &mut u64) -> i32 {
    // If the number is already 0
    if *bb == 0 { return -1; }
    // This function counts the number of 0 at the right of the LSO, which is equivalent to the position index of the LSO
    let lsb = bb.trailing_zeros() as i32;
    // We use Brian Kernighan trick -> "turn off" the least significant one
    *bb &= *bb - 1;
    lsb
}

// Generates a bitboard for one of the possible blocking bitboard of a concrete square
// Example: we have the bishop on e4. There are a lot of blocking possibilities. The index determines which one is the one we are going to calculate
// Because the mask in e4 is 9 (the bishop can move to 9 sqaures that can be blocked) the number of possible blocking convination is 2^9 = 512
// The combination of that 256 is the index. For the example we are going to simulate that index = 1.
// The for will iterate 9 times. The bit index will extract the index of the first blockeable square, that in this case, is 10
// Then the index will do a shift to the left of the number 0000 0000 ... 0001 of 0 units so the same number will be obtained.
// The if passes beacuse the and of the same number is the same number and because it has at least a 1 it will not be 0
// Then we create a mask with the bit on that index activated and will do an OR operation with the bloeckers bitboard and store in there the result
// so the new obstacle is represented with a 1. In this case for the other 8 iterations of the for because index only has 1 bit activated (is the number 1)
// no more obstacles will be painted. This is the most basic case. If we execute this function with the 0-511 (being 0 the board wihtout obstacles and 511 the board with all the squares with an obstacle)
// Resulting bitboard for index = 1:
//. . . . . . . .  <- Row 7 (Ignored)
//. . . . . . . .  
//. . . . . . . .
//. . . . . . . .  
//. . . . B . . .  <- The bishop (origin square)
//. . . . . . . .
//. . 1 . . . . .  
//. . . . . . . .  <- Row 0 (Ignored)

fn index_to_blockers(index: i32, bits: i32, mut mask: u64) -> u64 {
    let mut blockers = 0u64;
    for i in 0..bits {
        let bit_index = pop_lsb(&mut mask);
        if (index & (1 << i)) != 0 {
            blockers |= 1u64 << bit_index;
        }
    }
    blockers
}

// Generates the mask for the bishop movements omiting the border ones because they are the last 
//squares so even if they are occupied they wont interfiere in the diagonal of the other possible movements
// Example: for a bishop on e4:
//. . . . . . . .  <- Row 7 (Ignored)
//. 1 . . . 1 . .  <- Row 6 (Here the loop stops)
//. . 1 . 1 . . .
//. . . B . . . .  <- The bishop (origin square)
//. . 1 . 1 . . .
//. 1 . . . 1 . .
//. . . . . . . .  <- Row 1 (Here the loop stops)
//. . . . . . . .  <- Row 0 (Ignored)
fn bishop_mask(sq: i32) -> u64 {
    // The obstacle bitboard initially all 0
    let mut result = 0u64;
    // The row
    let target_r = sq / 8;
    // The column
    let target_f = sq % 8;

    // North east
    // Increase the row and column in 1 -> one diaginal square from the origin
    let (mut r, mut f) = (target_r + 1, target_f + 1);
    // Checks that the sqare is outside the upper and right borders
    while r <= 6 && f <= 6 { 
        // Shifts the 0000 0000 ... 0001 number to the diagonal up right quare -> the number of that cell is calculated by column + 8*row
        // And or operation is done with the original all 0 bitboard to activate the bit of the possible bishop movement
        result |= 1u64 << (f + r * 8); 
        r += 1; f += 1; 
    }
    // North west
    let (mut r, mut f) = (target_r + 1, target_f - 1);
    while r <= 6 && f >= 1 { result |= 1u64 << (f + r * 8); r += 1; f -= 1; }
    // South east
    let (mut r, mut f) = (target_r - 1, target_f + 1);
    while r >= 1 && f <= 6 { result |= 1u64 << (f + r * 8); r -= 1; f += 1; }
    // South west
    let (mut r, mut f) = (target_r - 1, target_f - 1);
    while r >= 1 && f >= 1 { result |= 1u64 << (f + r * 8); r -= 1; f -= 1; }

    result
}

fn rook_mask(sq: i32) -> u64 {
    // The obstacle bitboard initially all 0
    let mut result = 0u64;
    // The row
    let target_r = sq / 8;
    // The column
    let target_f = sq % 8;

    // North
    let (mut r, f) = (target_r + 1, target_f);
    while r <= 6 { result |= 1u64 << (f + r * 8); r += 1; }

    // South
    let (mut r, f) = (target_r - 1, target_f);
    while r > 0 { result |= 1u64 << (f + r * 8); r -= 1; }

    // East
    let (r, mut f) = (target_r, target_f + 1);
    while f <= 6 { result |= 1u64 << (f + r * 8); f += 1; }

    // West
    let (r, mut f) = (target_r, target_f - 1);
    while f > 0 { result |= 1u64 << (f + r * 8); f -= 1; }

    result
}

fn rook_attacks_bruteforce(sq: i32, blockers: u64) -> u64 {
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

// Given a blocker bitboard and a square it returns a bitboard with the actual possible movements
fn bishop_attacks_bruteforce(sq: i32, blockers: u64) -> u64 {
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

fn find_rook_magic(sq: i32, bits: i32, rng: &mut XorShift64) -> u64 {
    let mask = rook_mask(sq);

    let num_combinations = 1 << bits;

    let mut b_table = vec![0u64; num_combinations];
    let mut a_table = vec![0u64; num_combinations];

    for i in 0..num_combinations {
        b_table[i] = index_to_blockers(i as i32, bits, mask);
        a_table[i] = rook_attacks_bruteforce(sq, b_table[i]);
    }

    for _ in 0..10_000_000 {
        // Random number with small number of 1
        let magic = rng.few_bits();

        if count_bits((Wrapping(mask) * Wrapping(magic)).0 & 0xFF00000000000000) < 6 {
            continue;
        }

        let mut used = vec![0u64; num_combinations];
        // Check if the magic number is suitable
        let mut fail = false;

        for i in 0..num_combinations {
            // Hash indexador: we pultiply the magic number by the mask of the i index case and then we shift the bits to the
            // right in order to get the index by 64 - bits
            let index = ((Wrapping(b_table[i]) * Wrapping(magic)).0 >> (64 - bits)) as usize;
            
            // I that position on the index 0 it means it is not occupy
            if used[index] == 0 {
                used[index] = a_table[i]; // Asignamos si está libre
            } 
            else if used[index] != a_table[i] {
                fail = true; // destructive collision
                break;
            }
        }

        if !fail {
            return magic; // Magic number found
        }
    }

    panic!("No se pudo encontrar un número mágico para la casilla {}", sq);

}

// Find the magic number for a given square
// The magic number is no more than a key that allows us to obtain the obstacles of a bishop in a specific square
// For this purpose the magic number must give a different key for each square so we will have 64 magic numbers
fn find_bishop_magic(sq: i32, bits: i32, rng: &mut XorShift64) -> u64 {
    // Bitboard with all the squares a bishop in sq can occupy
    let mask = bishop_mask(sq);
    // This is equivalent to 2^bits nad because bits is the number of possible squares that the bishop of the sq can occupy
    // then it gives the number of total combinationd
    let num_combinations = 1 << bits;

    // b_table will be the bitboard with obstacles for the combination 0-(num_combinations - 1)
    let mut b_table = vec![0u64; num_combinations];
    let mut a_table = vec![0u64; num_combinations];
    for i in 0..num_combinations {
        b_table[i] = index_to_blockers(i as i32, bits, mask);
        a_table[i] = bishop_attacks_bruteforce(sq, b_table[i]);
    }

    // We try a big number of times untill no collisions are found
    for _ in 0..10_000_000 {
        // Random number with small number of 1
        let magic = rng.few_bits();
        
        // Optimization: discard the numbers that do not have bits on the MSO
        // Why? Because the objective of the magical number is to be a number that when multiply to the mask it elevates the
        // dispersed bits to the MSO in order to later do a shift to the right of 64 - bits and transform originally dispersed 1's in the index
        // of a matrix of 64 where the corresponding movement of the bishop is registered
        if count_bits((Wrapping(mask) * Wrapping(magic)).0 & 0xFF00000000000000) < 6 {
            continue;
        }

        // Array that simulates what would happen on a true game. It each of its bits would be an index with a 
        // possible obstacle bitboard
        let mut used = vec![0u64; num_combinations];
        // Check if the magic number is suitable
        let mut fail = false;

        for i in 0..num_combinations {
            // Hash indexador: we pultiply the magic number by the mask of the i index case and then we shift the bits to the
            // right in order to get the index by 64 - bits
            let index = ((Wrapping(b_table[i]) * Wrapping(magic)).0 >> (64 - bits)) as usize;
            
            // I that position on the index 0 it means it is not occupy
            if used[index] == 0 {
                used[index] = a_table[i]; // Asignamos si está libre
            } 
            
            // if used[index] != 0 it means that the cell is occupy with a bitboard, but we must check if the collision is constructive (the same legal movements)
            // or destructive (it is just trying to occupy another hole different position)
            // Example of constructive
            // A block mask says
            //. . . . . . . .  
            //. 1 . . . . . .  
            //. . 1 . . . . .
            //. . . B . . . .  
            //. . . . . . . .
            //. . . . . . . .
            //. . . . . . . .  
            //. . . . . . . .  
            // And a real movement says that the movement stops at C6 because the B7 will never be touched do to the previous obstacle
            // Both describe the same situation
            else if used[index] != a_table[i] {
                fail = true; // destructive collision
                break;
            }
        }

        if !fail {
            return magic; // Magic number found
        }
    }

    panic!("No se pudo encontrar un número mágico para la casilla {}", sq);
}

// Number of bits that a bishop can move from each square (from Tord Romstad)
const B_BITS: [i32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6
];

// Number of relevance mask bits for a rook on each square
pub const R_BITS: [i32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

fn main() {
    let mut rng = XorShift64::new(123456789); // Fixed seed for reproductibility, so the 64 magic number will be always be the same
    
    println!("pub const BISHOP_MAGICS: [u64; 64] = [");
    for square in 0..64 {
        let b_magic = find_bishop_magic(square, B_BITS[square as usize], &mut rng);
        // Print the magic number in hexadecimal for each square
        print!("  0x{:016x},", b_magic);
        if (square + 1) % 4 == 0 { println!(); }
    }
    println!("];");

    println!("pub const ROOK_MAGICS: [u64; 64] = [");
    for square in 0..64 {
        let r_magic = find_rook_magic(square, R_BITS[square as usize], &mut rng);
        // Print the magic number in hexadecimal for each square
        print!("  0x{:016x},", r_magic);
        if (square + 1) % 4 == 0 { println!(); }
    }
    println!("];");
}