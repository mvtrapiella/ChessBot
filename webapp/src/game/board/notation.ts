const FILES = 'abcdefgh'

// Backend square index (a1=0 .. h8=63, index = rank*8 + file) to UCI-style
// algebraic notation ("e2"), matching what board_backend's Position expects.
export function squareIndexToAlgebraic(squareIndex: number): string {
    const file = squareIndex % 8
    const rank = Math.floor(squareIndex / 8)
    return `${FILES[file]}${rank + 1}`
}
