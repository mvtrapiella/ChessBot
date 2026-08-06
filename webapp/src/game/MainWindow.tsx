import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import AppHeader from '../components/AppHeader'
import Board from './board/Board'
import { INITIAL_SQUARES } from './board/initialPosition'
import DepthSelector from './setup/DepthSelector'
import ColorSelector, { type PlayerColorChoice } from './setup/ColorSelector'
import { createGame, type ColorDTO } from '../api/gameApi'
import setupStyles from './setup/Setup.module.css'
import pageStyles from '../components/PageLayout.module.css'
import styles from './MainWindow.module.css'

function resolveColor(choice: PlayerColorChoice): ColorDTO {
    if (choice === 'random') {
        return Math.random() < 0.5 ? 'WHITE' : 'BLACK'
    }
    return choice === 'white' ? 'WHITE' : 'BLACK'
}

// Temporary: free movement, no legality (this board isn't backed by a real
// game yet). Real legal-move highlighting takes over once inside /game/:gameId.
function MainWindow() {
    const navigate = useNavigate()
    const [squares, setSquares] = useState<number[]>(INITIAL_SQUARES)
    const [selectedSquare, setSelectedSquare] = useState<number | null>(null)
    const [depth, setDepth] = useState(3)
    const [colorChoice, setColorChoice] = useState<PlayerColorChoice>('white')
    const [isStarting, setIsStarting] = useState(false)
    const [error, setError] = useState<string | null>(null)

    const movePiece = (origin: number, destination: number) => {
        if (origin === destination) {
            return
        }

        setSquares((current) => {
            const next = [...current]
            next[destination] = next[origin]
            next[origin] = 0
            return next
        })
        setSelectedSquare(null)
    }

    const handleSquareClick = (index: number) => {
        if (selectedSquare === null) {
            if (squares[index] !== 0) {
                setSelectedSquare(index)
            }
            return
        }

        if (index === selectedSquare) {
            setSelectedSquare(null)
            return
        }

        movePiece(selectedSquare, index)
    }

    const handlePlay = () => {
        setIsStarting(true)
        setError(null)

        createGame({ userColor: resolveColor(colorChoice), depth })
            .then((game) => navigate(`/game/${game.gameId}`))
            .catch((err: Error) => {
                setError(err.message)
                setIsStarting(false)
            })
    }

    return (
        <div className={pageStyles.page}>
            <AppHeader />

            <div className={pageStyles.content}>
                <Board
                    squares={squares}
                    selectedSquare={selectedSquare}
                    onSquareClick={handleSquareClick}
                    onPieceDrop={movePiece}
                />

                <div className={styles.panel}>
                    <DepthSelector depth={depth} onChange={setDepth} />
                    <ColorSelector selected={colorChoice} onChange={setColorChoice} />
                    <button
                        type="button"
                        className={setupStyles.playButton}
                        onClick={handlePlay}
                        disabled={isStarting}
                    >
                        {isStarting ? 'Starting…' : 'Play'}
                    </button>
                    {error && <p className={styles.error}>{error}</p>}
                </div>
            </div>
        </div>
    )
}

export default MainWindow
