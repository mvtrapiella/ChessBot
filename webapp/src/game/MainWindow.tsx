import { useState } from 'react'
import Board from './board/Board'
import { INITIAL_SQUARES } from './board/initialPosition'

// Temporary: free movement, no legality (this board isn't backed by a real
// game yet). Real legal-move highlighting takes over once inside /game/:gameId.
function MainWindow() {
    const [squares, setSquares] = useState<number[]>(INITIAL_SQUARES)
    const [selectedSquare, setSelectedSquare] = useState<number | null>(null)

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

    return (
        <Board
            squares={squares}
            selectedSquare={selectedSquare}
            onSquareClick={handleSquareClick}
        />
    )
}

export default MainWindow
