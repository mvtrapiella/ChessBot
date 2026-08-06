import { useRef, useState, type MouseEvent } from 'react'
import Cell from './Cell'
import './Board.css'

type BoardProps = {
    squares: number[]
    selectedSquare: number | null
    highlightedSquares?: number[]
    onSquareClick: (index: number) => void
}

type Arrow = {
    from: number
    to: number
}

// Cell 0 in the visual grid (top-left) is a8; backend square indices run
// a1=0 .. h8=63 (index = rank*8 + file). This converts one to the other.
function gridPositionToSquareIndex(position: number): number {
    const row = Math.floor(position / 8)
    const col = position % 8
    const rank = 7 - row
    return rank * 8 + col
}

// Center of a square in the SVG overlay's 8x8 coordinate space.
function squareIndexToCenter(squareIndex: number) {
    const rank = Math.floor(squareIndex / 8)
    const file = squareIndex % 8
    const row = 7 - rank
    return { x: file + 0.5, y: row + 0.5 }
}

const RIGHT_BUTTON = 2

function Board({ squares, selectedSquare, highlightedSquares = [], onSquareClick }: Readonly<BoardProps>) {
    const [markedSquares, setMarkedSquares] = useState<Set<number>>(new Set())
    const [arrows, setArrows] = useState<Arrow[]>([])
    const rightDragOrigin = useRef<number | null>(null)

    const clearAnnotations = () => {
        setMarkedSquares(new Set())
        setArrows([])
    }

    const handleClick = (squareIndex: number) => {
        clearAnnotations()
        onSquareClick(squareIndex)
    }

    const handleMouseDown = (squareIndex: number) => (event: MouseEvent) => {
        if (event.button === RIGHT_BUTTON) {
            rightDragOrigin.current = squareIndex
        }
    }

    const handleMouseUp = (squareIndex: number) => (event: MouseEvent) => {
        if (event.button !== RIGHT_BUTTON || rightDragOrigin.current === null) {
            return
        }

        const origin = rightDragOrigin.current
        rightDragOrigin.current = null

        if (origin === squareIndex) {
            setMarkedSquares((current) => {
                const next = new Set(current)
                if (next.has(squareIndex)) {
                    next.delete(squareIndex)
                } else {
                    next.add(squareIndex)
                }
                return next
            })
        } else {
            setArrows((current) => [...current, { from: origin, to: squareIndex }])
        }
    }

    const gridPositions = Array.from({ length: 64 }, (_, position) => position)

    return (
        <div className="board" onContextMenu={(event) => event.preventDefault()}>
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
                        isMarked={markedSquares.has(squareIndex)}
                        onClick={() => handleClick(squareIndex)}
                        onMouseDown={handleMouseDown(squareIndex)}
                        onMouseUp={handleMouseUp(squareIndex)}
                    />
                )
            })}

            <svg className="board-arrows" viewBox="0 0 8 8">
                <defs>
                    <marker
                        id="arrowhead"
                        viewBox="0 0 10 10"
                        refX="8"
                        refY="5"
                        markerWidth="0.6"
                        markerHeight="0.6"
                        markerUnits="userSpaceOnUse"
                        orient="auto-start-reverse"
                    >
                        <path d="M0,0 L10,5 L0,10 z" fill="#e08e3e" />
                    </marker>
                </defs>
                {arrows.map((arrow, index) => {
                    const from = squareIndexToCenter(arrow.from)
                    const to = squareIndexToCenter(arrow.to)
                    return (
                        <line
                            key={index}
                            x1={from.x}
                            y1={from.y}
                            x2={to.x}
                            y2={to.y}
                            stroke="#e08e3e"
                            strokeWidth={0.15}
                            markerEnd="url(#arrowhead)"
                        />
                    )
                })}
            </svg>
        </div>
    )
}

export default Board
