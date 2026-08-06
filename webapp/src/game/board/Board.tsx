import { useEffect, useRef, useState, type MouseEvent as ReactMouseEvent } from 'react'
import Cell from './Cell'
import { PIECE_ASSETS } from './pieceAssets'
import styles from './Board.module.css'

export type Perspective = 'white' | 'black'

type BoardProps = {
    squares: number[]
    selectedSquare: number | null
    highlightedSquares?: number[]
    perspective?: Perspective
    onSquareClick: (index: number) => void
    // Optional: enables mouse drag-to-move. Called on drop with the origin
    // and destination square; the caller decides whether that's a legal
    // move (same validation path as clicking origin then destination).
    onPieceDrop?: (from: number, to: number) => void
}

type Arrow = {
    from: number
    to: number
}

type MoveAnimation = {
    from: number
    to: number
}

type DragState = {
    origin: number
    piece: number
    x: number
    y: number
}

const RIGHT_BUTTON = 2
const LEFT_BUTTON = 0
const DRAG_THRESHOLD_PX = 4
const MOVE_ANIMATION_MS = 300

// Cell 0 in the visual grid (top-left) is a8 when viewed as White (h1 when
// viewed as Black); backend square indices run a1=0 .. h8=63
// (index = rank*8 + file). This converts one to the other.
function gridPositionToSquareIndex(position: number, perspective: Perspective): number {
    const row = Math.floor(position / 8)
    const col = position % 8
    const rank = perspective === 'white' ? 7 - row : row
    const file = perspective === 'white' ? col : 7 - col
    return rank * 8 + file
}

function squareIndexToGridPosition(squareIndex: number, perspective: Perspective) {
    const rank = Math.floor(squareIndex / 8)
    const file = squareIndex % 8
    const row = perspective === 'white' ? 7 - rank : rank
    const col = perspective === 'white' ? file : 7 - file
    return { row, col }
}

// Center of a square in the SVG overlay's 8x8 coordinate space.
function squareIndexToCenter(squareIndex: number, perspective: Perspective) {
    const { row, col } = squareIndexToGridPosition(squareIndex, perspective)
    return { x: col + 0.5, y: row + 0.5 }
}

// Diffs two square arrays into (from, to) piece movements, so only the
// pieces that actually moved animate rather than the whole board snapping.
// Matches same-piece-code departures/arrivals first (normal moves, captures,
// castling's rook), then pairs up whatever's left positionally (promotion,
// where the piece code itself changes) or leaves it unpaired (a captured
// piece has nowhere to "arrive", so it just disappears -- no animation).
function diffMoves(previous: number[], next: number[]): MoveAnimation[] {
    const departures: number[] = []
    const arrivals: number[] = []

    for (let i = 0; i < 64; i++) {
        if (previous[i] !== 0 && next[i] === 0) departures.push(i)
        if (next[i] !== 0 && next[i] !== previous[i]) arrivals.push(i)
    }

    const pairs: MoveAnimation[] = []
    const usedArrivals = new Set<number>()

    for (const from of departures) {
        const to = arrivals.find((a) => !usedArrivals.has(a) && next[a] === previous[from])
        if (to !== undefined) {
            pairs.push({ from, to })
            usedArrivals.add(to)
        }
    }

    const leftoverDepartures = departures.filter((from) => !pairs.some((p) => p.from === from))
    const leftoverArrivals = arrivals.filter((a) => !usedArrivals.has(a))
    leftoverDepartures.forEach((from, i) => {
        const to = leftoverArrivals[i]
        if (to !== undefined) pairs.push({ from, to })
    })

    return pairs
}

function Board({
    squares,
    selectedSquare,
    highlightedSquares = [],
    perspective = 'white',
    onSquareClick,
    onPieceDrop,
}: Readonly<BoardProps>) {
    const [markedSquares, setMarkedSquares] = useState<Set<number>>(new Set())
    const [arrows, setArrows] = useState<Arrow[]>([])
    const rightDragOrigin = useRef<number | null>(null)

    const boardRef = useRef<HTMLDivElement>(null)
    const previousSquaresRef = useRef(squares)
    const [moveAnimations, setMoveAnimations] = useState<MoveAnimation[]>([])

    const dragCandidateRef = useRef<{ origin: number; piece: number; startX: number; startY: number } | null>(null)
    const [drag, setDrag] = useState<DragState | null>(null)

    // Detect which pieces actually moved between renders, so only those
    // slide instead of every square just snapping to its new contents.
    useEffect(() => {
        if (previousSquaresRef.current === squares) {
            return
        }
        previousSquaresRef.current = squares

        const pairs = diffMoves(previousSquaresRef.current, squares)
        if (pairs.length === 0 || pairs.length > 2) {
            setMoveAnimations([])
            return
        }

        setMoveAnimations(pairs)
        const timeout = setTimeout(() => setMoveAnimations([]), MOVE_ANIMATION_MS)
        return () => clearTimeout(timeout)
    }, [squares])

    const clearAnnotations = () => {
        setMarkedSquares(new Set())
        setArrows([])
    }

    const handleClick = (squareIndex: number) => {
        clearAnnotations()
        onSquareClick(squareIndex)
    }

    const handleMouseDown = (squareIndex: number) => (event: ReactMouseEvent) => {
        if (event.button === RIGHT_BUTTON) {
            rightDragOrigin.current = squareIndex
            return
        }

        if (event.button === LEFT_BUTTON && onPieceDrop && squares[squareIndex] !== 0) {
            dragCandidateRef.current = {
                origin: squareIndex,
                piece: squares[squareIndex],
                startX: event.clientX,
                startY: event.clientY,
            }
        }
    }

    const handleMouseUp = (squareIndex: number) => (event: ReactMouseEvent) => {
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

    // Drag-to-move is tracked at the window level, since the pointer moves
    // well outside the origin cell's bounds during a real drag.
    useEffect(() => {
        if (!onPieceDrop) {
            return
        }

        const handleWindowMouseMove = (event: globalThis.MouseEvent) => {
            const candidate = dragCandidateRef.current
            if (!candidate) {
                return
            }

            if (!drag) {
                const dx = event.clientX - candidate.startX
                const dy = event.clientY - candidate.startY
                if (Math.hypot(dx, dy) < DRAG_THRESHOLD_PX) {
                    return
                }
                // Reuse the normal "select" side effect (highlighting legal
                // moves etc.) -- unless it's already selected, in which case
                // doing this again would toggle it back off.
                if (selectedSquare !== candidate.origin) {
                    onSquareClick(candidate.origin)
                }
            }

            setDrag({ origin: candidate.origin, piece: candidate.piece, x: event.clientX, y: event.clientY })
        }

        const handleWindowMouseUp = (event: globalThis.MouseEvent) => {
            const candidate = dragCandidateRef.current
            dragCandidateRef.current = null

            const wasDragging = drag !== null
            setDrag(null)

            if (!candidate || !wasDragging) {
                return
            }

            const target = document
                .elementFromPoint(event.clientX, event.clientY)
                ?.closest('[data-square]') as HTMLElement | null
            const targetSquare = target ? Number(target.dataset.square) : null

            if (targetSquare !== null && targetSquare !== candidate.origin) {
                onPieceDrop(candidate.origin, targetSquare)
            }
        }

        window.addEventListener('mousemove', handleWindowMouseMove)
        window.addEventListener('mouseup', handleWindowMouseUp)
        return () => {
            window.removeEventListener('mousemove', handleWindowMouseMove)
            window.removeEventListener('mouseup', handleWindowMouseUp)
        }
    }, [drag, onPieceDrop, onSquareClick, selectedSquare])

    const gridPositions = Array.from({ length: 64 }, (_, position) => position)
    const cellSize = drag && boardRef.current ? boardRef.current.getBoundingClientRect().width / 8 : 0
    const dragAsset = drag ? PIECE_ASSETS[drag.piece] : undefined

    return (
        <div className={styles.board} ref={boardRef} onContextMenu={(event) => event.preventDefault()}>
            {gridPositions.map((position) => {
                const squareIndex = gridPositionToSquareIndex(position, perspective)
                const row = Math.floor(squareIndex / 8)
                const col = squareIndex % 8
                const isLight = (row + col) % 2 === 1

                const animation = moveAnimations.find((a) => a.to === squareIndex)
                let animateFrom: { dx: number; dy: number } | null = null
                if (animation) {
                    const from = squareIndexToGridPosition(animation.from, perspective)
                    const to = squareIndexToGridPosition(squareIndex, perspective)
                    animateFrom = { dx: from.col - to.col, dy: from.row - to.row }
                }

                return (
                    <Cell
                        key={squareIndex}
                        piece={drag?.origin === squareIndex ? 0 : squares[squareIndex]}
                        isLight={isLight}
                        isSelected={selectedSquare === squareIndex}
                        isHighlighted={highlightedSquares.includes(squareIndex)}
                        isMarked={markedSquares.has(squareIndex)}
                        squareIndex={squareIndex}
                        animateFrom={animateFrom}
                        onClick={() => handleClick(squareIndex)}
                        onMouseDown={handleMouseDown(squareIndex)}
                        onMouseUp={handleMouseUp(squareIndex)}
                    />
                )
            })}

            <svg className={styles.arrows} viewBox="0 0 8 8">
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
                    const from = squareIndexToCenter(arrow.from, perspective)
                    const to = squareIndexToCenter(arrow.to, perspective)
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

            {drag && dragAsset && cellSize > 0 && (
                <img
                    src={dragAsset}
                    alt=""
                    className={styles.draggedPiece}
                    style={{
                        width: cellSize,
                        height: cellSize,
                        left: drag.x - cellSize / 2,
                        top: drag.y - cellSize / 2,
                    }}
                />
            )}
        </div>
    )
}

export default Board
