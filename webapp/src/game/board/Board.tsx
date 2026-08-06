import Cell from './Cell'
import './Board.css'

type BoardProps = {
    squares: number[]
    selectedSquare: number | null
    highlightedSquares?: number[]
    onSquareClick: (index: number) => void
}

// Cell 0 in the visual grid (top-left) is a8; backend square indices run
// a1=0 .. h8=63 (index = rank*8 + file). This converts one to the other.
function gridPositionToSquareIndex(position: number): number {
    const row = Math.floor(position / 8)
    const col = position % 8
    const rank = 7 - row
    return rank * 8 + col
}

function Board({ squares, selectedSquare, highlightedSquares = [], onSquareClick }: BoardProps) {
    const gridPositions = Array.from({ length: 64 }, (_, position) => position)

    return (
        <div className="board">
            {gridPositions.map((position) => {
                const squareIndex = gridPositionToSquareIndex(position)
                const row = Math.floor(squareIndex / 8)
                const col = squareIndex % 8
                const isLight = (row + col) % 2 === 1

                return (
                    <Cell
                        key={squareIndex}
                        piece={squares[squareIndex]}
                        isLight={isLight}
                        isSelected={selectedSquare === squareIndex}
                        isHighlighted={highlightedSquares.includes(squareIndex)}
                        onClick={() => onSquareClick(squareIndex)}
                    />
                )
            })}
        </div>
    )
}

export default Board
