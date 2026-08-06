import { useState } from 'react'
import AppHeader from '../components/AppHeader'
import Board from './board/Board'
import { INITIAL_SQUARES } from './board/initialPosition'
import DepthSelector from './setup/DepthSelector'
import ColorSelector, { type PlayerColorChoice } from './setup/ColorSelector'
import setupStyles from './setup/Setup.module.css'
import styles from './MainWindow.module.css'

// Temporary: free movement, no legality (this board isn't backed by a real
// game yet). Real legal-move highlighting takes over once inside /game/:gameId.
function MainWindow() {
    const [squares, setSquares] = useState<number[]>(INITIAL_SQUARES)
    const [selectedSquare, setSelectedSquare] = useState<number | null>(null)
    const [depth, setDepth] = useState(3)
    const [colorChoice, setColorChoice] = useState<PlayerColorChoice>('white')

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

        setSquares((current) => {
            const next = [...current]
            next[index] = next[selectedSquare]
            next[selectedSquare] = 0
            return next
        })
        setSelectedSquare(null)
    }

    const handlePlay = () => {
        // TODO: POST /games with { userColor: resolve(colorChoice), depth }, then navigate to /game/:gameId
        console.log('Play requested', { depth, colorChoice })
    }

    return (
        <div className={styles.page}>
            <AppHeader />

            <div className={styles.content}>
                <Board
                    squares={squares}
                    selectedSquare={selectedSquare}
                    onSquareClick={handleSquareClick}
                />

                <div className={styles.panel}>
                    <DepthSelector depth={depth} onChange={setDepth} />
                    <ColorSelector selected={colorChoice} onChange={setColorChoice} />
                    <button type="button" className={setupStyles.playButton} onClick={handlePlay}>
                        Play
                    </button>
                </div>
            </div>
        </div>
    )
}

export default MainWindow
