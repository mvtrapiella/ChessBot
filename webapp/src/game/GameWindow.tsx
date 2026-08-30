import { useEffect, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import AppHeader from '../components/AppHeader'
import Modal from '../components/Modal'
import Board from './board/Board'
import { INITIAL_SQUARES } from './board/initialPosition'
import { squareIndexToAlgebraic } from './board/notation'
import MoveHistory from './MoveHistory'
import {
    getGame,
    getLegalMoves,
    makeMove,
    undoMove,
    type ColorDTO,
    type GameStateDTO,
} from '../api/gameApi'
import pageStyles from '../components/PageLayout.module.css'
import setupStyles from './setup/Setup.module.css'
import styles from './GameWindow.module.css'

// Matches board_backend's piece codes (types.rs): 1-6 = white, 7-12 = black
const WHITE_PAWN = 1
const BLACK_PAWN = 7
const WHITE_QUEEN = 5
const BLACK_QUEEN = 11
const WHITE_KING = 6
const BLACK_KING = 12

const ILLEGAL_MOVE_FLASH_MS = 750

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

function drawReasonMessage(game: GameStateDTO): string {
    switch (game.drawReason) {
        case 'STALEMATE':
            return 'By stalemate'
        case 'FIFTY_MOVE_RULE':
            return 'By the 50-move rule'
        case 'INSUFFICIENT_MATERIAL':
            return 'By insufficient material'
        case 'THREEFOLD_REPETITION':
            return 'By threefold repetition'
        default:
            return ''
    }
}

// Headline for the end-of-game modal. "Won" always means the *bot's* color
// won -- it's the human's own color that decides whether that reads as a
// win or a loss for them.
function endGameHeadline(game: GameStateDTO, surrendered: boolean): string {
    if (surrendered) {
        return 'Bot wins'
    }
    switch (game.status) {
        case 'WHITE_WON':
            return game.userColor === 'WHITE' ? 'You win!' : 'Bot wins'
        case 'BLACK_WON':
            return game.userColor === 'BLACK' ? 'You win!' : 'Bot wins'
        case 'DRAWN':
            return 'Draw'
        default:
            return ''
    }
}

function GameWindow() {
    const { gameId } = useParams<{ gameId: string }>()
    const navigate = useNavigate()
    const [game, setGame] = useState<GameStateDTO | null>(null)
    const [viewIndex, setViewIndex] = useState(-1)
    const [selectedSquare, setSelectedSquare] = useState<number | null>(null)
    const [highlightedSquares, setHighlightedSquares] = useState<number[]>([])
    const [flashSquare, setFlashSquare] = useState<number | null>(null)
    const [error, setError] = useState<string | null>(null)
    const [loading, setLoading] = useState(true)
    const [surrendered, setSurrendered] = useState(false)

    useEffect(() => {
        if (!gameId) return

        let cancelled = false
        setLoading(true)

        getGame(gameId)
            .then((data) => {
                if (cancelled) return
                setGame(data)
                setViewIndex(data.moveHistory.length - 1)
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

    const isLive = game !== null && viewIndex === game.moveHistory.length - 1
    const displaySquares =
        !game || isLive
            ? (game?.squares ?? INITIAL_SQUARES)
            : viewIndex === -1
              ? INITIAL_SQUARES
              : game.moveHistory[viewIndex].squares

    // Only meaningful for the live position -- inCheck describes game.squares, not
    // whatever historical position the move-history scrubber might be showing.
    const checkedSquare =
        isLive && game && game.inCheck
            ? game.squares.indexOf(game.sideToMove === 'WHITE' ? WHITE_KING : BLACK_KING)
            : null

    const deselect = () => {
        setSelectedSquare(null)
        setHighlightedSquares([])
    }

    const jumpTo = (index: number) => {
        deselect()
        setViewIndex(index)
    }

    const selectSquare = (index: number) => {
        if (!gameId) return

        setSelectedSquare(index)
        setHighlightedSquares([])
        getLegalMoves(gameId, squareIndexToAlgebraic(index))
            .then(setHighlightedSquares)
            .catch(() => setHighlightedSquares([]))
    }

    // Shared by click-to-move and drag-to-move. Drag bypasses the selection
    // state machine entirely, so it re-checks the same turn/ownership rule
    // that click's "canSelect" already gates on -- otherwise dragging the
    // opponent's piece would optimistically move it before the backend
    // rejects it.
    const attemptMove = (origin: number, destination: number) => {
        if (!game || !gameId || origin === destination) {
            return
        }

        const canMove =
            pieceColorOf(game.squares[origin]) === game.userColor && game.sideToMove === game.userColor
        if (!canMove) {
            return
        }

        const promotion = promotionFor(game.squares[origin], destination)
        const previousGame = game

        deselect()
        setError(null)
        setGame(applyMoveLocally(game, origin, destination))

        makeMove(gameId, {
            origin: squareIndexToAlgebraic(origin),
            destination: squareIndexToAlgebraic(destination),
            promotion,
        })
            .then((data) => {
                setGame(data)
                setViewIndex(data.moveHistory.length - 1)
            })
            .catch(() => {
                setGame(previousGame)
                setViewIndex(previousGame.moveHistory.length - 1)
                setFlashSquare(origin)
                setTimeout(() => {
                    setFlashSquare((current) => (current === origin ? null : current))
                }, ILLEGAL_MOVE_FLASH_MS)
            })
    }

    const handleSquareClick = (index: number) => {
        if (!game || !gameId || !isLive || game.status !== 'IN_PROGRESS') {
            return
        }

        const canSelect =
            pieceColorOf(game.squares[index]) === game.userColor && game.sideToMove === game.userColor

        if (selectedSquare === null) {
            if (canSelect) {
                selectSquare(index)
            }
            return
        }

        if (index === selectedSquare) {
            deselect()
            return
        }

        if (canSelect) {
            selectSquare(index)
            return
        }

        attemptMove(selectedSquare, index)
    }

    const handleUndo = () => {
        if (!gameId) return

        setError(null)
        undoMove(gameId)
            .then((data) => {
                deselect()
                setGame(data)
                setViewIndex(data.moveHistory.length - 1)
            })
            .catch((err: Error) => setError(err.message))
    }

    const canUndo = game !== null && game.moveHistory.length >= 2

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
                            squares={displaySquares}
                            selectedSquare={isLive ? selectedSquare : null}
                            highlightedSquares={isLive ? highlightedSquares : []}
                            checkedSquare={checkedSquare}
                            flashSquare={isLive ? flashSquare : null}
                            perspective={game.userColor === 'BLACK' ? 'black' : 'white'}
                            onSquareClick={handleSquareClick}
                            onPieceDrop={isLive && game.status === 'IN_PROGRESS' ? attemptMove : undefined}
                        />

                        <div className={styles.panel}>
                            <span className={styles.label}>Playing as</span>
                            <p className={styles.value}>{game.userColor === 'WHITE' ? 'White' : 'Black'}</p>

                            <span className={styles.label}>Turn</span>
                            <p className={styles.value}>{game.sideToMove === 'WHITE' ? 'White' : 'Black'}</p>

                            {error && <p className={styles.error}>{error}</p>}

                            <span className={styles.label}>Moves</span>
                            <MoveHistory moveHistory={game.moveHistory} viewIndex={viewIndex} onSelectIndex={jumpTo} />

                            <div className={styles.rewindRow}>
                                <button
                                    type="button"
                                    className={styles.rewindButton}
                                    onClick={() => jumpTo(-1)}
                                    disabled={viewIndex === -1}
                                    aria-label="Rewind to the first move"
                                >
                                    |&lt;
                                </button>
                                <button
                                    type="button"
                                    className={styles.rewindButton}
                                    onClick={() => jumpTo(Math.max(-1, viewIndex - 1))}
                                    disabled={viewIndex === -1}
                                    aria-label="Step back one move"
                                >
                                    &lt;
                                </button>
                                <button
                                    type="button"
                                    className={styles.rewindButton}
                                    onClick={() => jumpTo(Math.min(game.moveHistory.length - 1, viewIndex + 1))}
                                    disabled={isLive}
                                    aria-label="Step forward one move"
                                >
                                    &gt;
                                </button>
                            </div>

                            <button
                                type="button"
                                className={styles.undoButton}
                                onClick={handleUndo}
                                disabled={!canUndo}
                            >
                                Undo last turn
                            </button>

                            <button
                                type="button"
                                className={styles.surrenderButton}
                                onClick={() => setSurrendered(true)}
                                disabled={game.status !== 'IN_PROGRESS'}
                            >
                                Surrender
                            </button>
                        </div>
                    </>
                )}

                {!loading && !game && error && <p className={styles.error}>{error}</p>}
            </div>

            {game && (
                <Modal open={surrendered || game.status !== 'IN_PROGRESS'}>
                    <p className={styles.endGameTitle}>{endGameHeadline(game, surrendered)}</p>
                    {!surrendered && game.status === 'DRAWN' && (
                        <p className={styles.endGameSubtitle}>{drawReasonMessage(game)}</p>
                    )}
                    <button
                        type="button"
                        className={`${setupStyles.playButton} ${styles.returnButton}`}
                        onClick={() => navigate('/')}
                    >
                        Return
                    </button>
                </Modal>
            )}
        </div>
    )
}

export default GameWindow
