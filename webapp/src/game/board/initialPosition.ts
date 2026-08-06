// Standard chess starting position, in the same flat, 64-square format
// board_backend's GameStateDTO.squares uses (index = rank*8 + file, a1 = 0, h8 = 63).
export const INITIAL_SQUARES: number[] = [
    2, 3, 4, 5, 6, 4, 3, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    7, 7, 7, 7, 7, 7, 7, 7,
    8, 9, 10, 11, 12, 10, 9, 8,
]
