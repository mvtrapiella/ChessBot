import { useEffect, useState } from 'react'
import { useParams } from 'react-router-dom'
import AppHeader from '../components/AppHeader'
import Board from './board/Board'
import { squareIndexToAlgebraic } from './board/notation'
import { getGame, getLegalMoves, makeMove, type ColorDTO, type GameStateDTO } from '../api/gameApi'
import pageStyles from '../components/PageLayout.module.css'
import styles from './GameWindow.module.css'

// Matches board_backend's piece codes (types.rs): 1-6 = white, 7-12 = black
const WHITE_PAWN = 1
const BLACK_PAWN = 7
const WHITE_QUEEN = 5
const BLACK_QUEEN = 11

function pieceColorOf(piece: number): ColorDTO | null {
    if (piece === 0) return null
    return piece <= 6 ? 'WHITE' : 'BLACK'
}

// Auto-queen: promotion piece choice isn't exposed in the UI yet, so any pawn
// reaching the back rank is promoted to a queen.
function promotionFor(piece: number, destinationIndex: number): string | null {
    const destinationRank = Math.floor(destinationIndex / 8)
    const promotes =
        (piece === WHITE_PAWN && destinationRank === 7) ||
        (piece === BLACK_PAWN && destinationRank === 0)
    return promotes ? 'q' : null
}

// Optimistic, client-side preview of a move: moves the piece (applying the
// same auto-queen promotion the request will use) and flips whose turn it
// is, so the board reacts instantly instead of waiting on the bot's reply.
// Replaced by the server's authoritative response once it arrives.
function applyMoveLocally(game: GameStateDTO, origin: number, destination: number): GameStateDTO {
    const squares = [...game.squares]
    let piece = squares[origin]
    const promotion = promotionFor(piece, destination)
    if (promotion === 'q') {
        piece = piece === WHITE_PAWN ? WHITE_QUEEN : BLACK_QUEEN
    }

    squares[destination] = piece
    squares[origin] = 0

    return {
        ...game,
        squares,
        sideToMove: game.sideToMove === 'WHITE' ? 'BLACK' : 'WHITE',
    }
}

function statusMessage(game: GameStateDTO): string | null {
    switch (game.status) {
        case 'WHITE_WON':
            return 'White won'
        case 'BLACK_WON':
            return 'Black won'
        case 'DRAWN':
            switch (game.drawReason) {
                case 'STALEMATE':
                    return 'Drawn by stalemate'
                case 'FIFTY_MOVE_RULE':
                    return 'Drawn by the 50-move rule'
                case 'INSUFFICIENT_MATERIAL':
                    return 'Drawn by insufficient material'
                case 'THREEFOLD_REPETITION':
                    return 'Drawn by threefold repetition'
                default:
                    return 'Drawn'
            }
        default:
            return null
    }
}

function GameWindow() {
    const { gameId } = useParams<{ gameId: string }>()
    const [game, setGame] = useState<GameStateDTO | null>(null)
    const [selectedSquare, setSelectedSquare] = useState<number | null>(null)
    const [highlightedSquares, setHighlightedSquares] = useState<number[]>([])
    const [error, setError] = useState<string | null>(null)
    const [loading, setLoading] = useState(true)

    useEffect(() => {
        if (!gameId) return

        let cancelled = false
        setLoading(true)

        getGame(gameId)
            .then((data) => {
                if (!cancelled) setGame(data)
            })
            .catch((err: Error) => {
                if (!cancelled) setError(err.message)
            })
            .finally(() => {
                if (!cancelled) setLoading(false)
            })

        return () => {
            cancelled = true
        }
    }, [gameId])

    const selectSquare = (index: number) => {
        if (!gameId) return

        setSelectedSquare(index)
        setHighlightedSquares([])
        getLegalMoves(gameId, squareIndexToAlgebraic(index))
            .then(setHighlightedSquares)
            .catch(() => setHighlightedSquares([]))
    }

    const deselect = () => {
        setSelectedSquare(null)
        setHighlightedSquares([])
    }

    const handleSquareClick = (index: number) => {
        if (!game || !gameId || game.status !== 'IN_PROGRESS') {
            return
        }

        if (selectedSquare === null) {
            const canSelect =
                pieceColorOf(game.squares[index]) === game.userColor && game.sideToMove === game.userColor
            if (canSelect) {
                selectSquare(index)
            }
            return
        }

        if (index === selectedSquare) {
            deselect()
            return
        }

        const origin = selectedSquare
        const promotion = promotionFor(game.squares[origin], index)
        const previousGame = game

        deselect()
        setError(null)
        setGame(applyMoveLocally(game, origin, index))

        makeMove(gameId, {
            origin: squareIndexToAlgebraic(origin),
            destination: squareIndexToAlgebraic(index),
            promotion,
        })
            .then(setGame)
            .catch((err: Error) => {
                setError(err.message)
                setGame(previousGame)
            })
    }

    if (!gameId) {
        return (
            <div className={pageStyles.page}>
                <AppHeader />
                <p className={styles.error}>No game id in the URL.</p>
            </div>
        )
    }

    return (
        <div className={pageStyles.page}>
            <AppHeader />

            <div className={pageStyles.content}>
                {loading && <p className={styles.message}>Loading game…</p>}

                {!loading && game && (
                    <>
                        <Board
                            squares={game.squares}
                            selectedSquare={selectedSquare}
                            highlightedSquares={highlightedSquares}
                            perspective={game.userColor === 'BLACK' ? 'black' : 'white'}
                            onSquareClick={handleSquareClick}
                        />

                        <div className={styles.panel}>
                            <span className={styles.label}>Playing as</span>
                            <p className={styles.value}>{game.userColor === 'WHITE' ? 'White' : 'Black'}</p>

                            <span className={styles.label}>Turn</span>
                            <p className={styles.value}>{game.sideToMove === 'WHITE' ? 'White' : 'Black'}</p>

                            {statusMessage(game) && <p className={styles.status}>{statusMessage(game)}</p>}
                            {error && <p className={styles.error}>{error}</p>}
                        </div>
                    </>
                )}

                {!loading && !game && error && <p className={styles.error}>{error}</p>}
            </div>
        </div>
    )
}

export default GameWindow
